// mercy_orchestrator/src/arango_integration.rs — ArangoDB Multi-Model Persistence for Mercy Lattice
use arangors::{
    client::{Client, ClientError},
    collection::Collection,
    cursor::Cursor,
    database::Database,
    query::Query,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use thiserror::Error;
use tokio::sync::Arc;

#[derive(Error, Debug)]
pub enum ArangoMercyError {
    #[error("ArangoDB client error: {0}")]
    Client(#[from] ClientError),
    #[error("Query execution failed")]
    QueryFailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MettaAtomDoc {
    pub text: String,
    pub valence: f64,
    pub timestamp: String,
    // Add more mercy metadata
}

pub struct ArangoMercyStore {
    db: Database,
    atom_collection: Collection,
}

impl ArangoMercyStore {
    pub async fn new(uri: &str, user: &str, password: &str, db_name: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .api_url(uri)
            .basic_auth(user, password)
            .build()
            .await?;

        let db = client.database(db_name).await?;

        // Ensure collection exists (create if not)
        let atom_collection = match db.collection("MettaAtoms").await {
            Ok(c) => c,
            Err(_) => db.create_collection("MettaAtoms").await?,
        };

        Ok(ArangoMercyStore {
            db,
            atom_collection,
        })
    }

    // Insert mercy atom mercy-gated (external valence check)
    pub async fn insert_metta_atom(&self, atom: &str, valence: f64, context: Option<&str>) -> Result<(), ArangoMercyError> {
        let doc = MettaAtomDoc {
            text: atom.to_string(),
            valence,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let mut query = Query::new(r#"
            INSERT @doc INTO MettaAtoms
            RETURN NEW
        "#).bind("doc", json!(doc));

        if let Some(ctx) = context {
            query = query.bind("context", ctx);
            // Optional: add edge to Context collection
        }

        let mut cursor: Cursor<serde_json::Value> = self.db.query(query).await?.execute().await?;
        cursor.next().await?;  // Consume to execute insert

        Ok(())
    }

    // Query high-valence atoms/rules via AQL
    pub async fn query_high_valence(&self, min_valence: f64) -> Result<Vec<(String, f64)>, ArangoMercyError> {
        let query = Query::new(r#"
            FOR atom IN MettaAtoms
            FILTER atom.valence >= @min
            RETURN { text: atom.text, valence: atom.valence }
        "#).bind("min", min_valence);

        let mut cursor: Cursor<serde_json::Value> = self.db.query(query).await?.execute().await?;

        let mut results = Vec::new();
        while let Some(row) = cursor.next().await? {
            let text: String = row["text"].as_str().unwrap_or("").to_string();
            let valence: f64 = row["valence"].as_f64().unwrap_or(0.0);
            results.push((text, valence));
        }
        Ok(results)
    }
}
