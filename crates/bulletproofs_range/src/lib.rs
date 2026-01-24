//! BulletproofsRange — Logarithmic Range + Aggregation Verification
//! Ultramasterful Halo2 gadget for aggregated proof verification

use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, Value},
    plonk::{ConstraintSystem, Error},
};
use halo2_gadgets::bulletproofs::aggregation::BulletproofAggregationChip;
use pasta_curves::pallas::Scalar;

pub struct BulletproofAggregationVerifier {
    config: BulletproofAggregationConfig,
}

impl BulletproofAggregationVerifier {
    pub fn configure(meta: &mut ConstraintSystem<Scalar>) -> BulletproofAggregationConfig {
        BulletproofAggregationChip::configure(meta)
    }

    pub fn construct(config: BulletproofAggregationConfig) -> Self {
        Self { config }
    }

    /// Verify aggregated Bulletproofs (multiple range proofs)
    pub fn verify_aggregated_proof(
        &self,
        layouter: impl Layouter<Scalar>,
        aggregated_proof: Value<&RangeProof>,
        public_inputs: &[Value<Scalar>],
    ) -> Result<(), Error> {
        let aggregation = BulletproofAggregationChip::construct(self.config.clone());
        aggregation.verify(layouter.namespace(|| "aggregated_bulletproofs"), aggregated_proof, public_inputs)
    }
}
