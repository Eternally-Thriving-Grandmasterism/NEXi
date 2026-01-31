// mercy_swarm_replication/src/lib.rs — Exponential Lunar Mining Swarm
#[derive(Debug, Clone)]
pub struct SwarmUnit {
    pub generation: u32,
    pub capacity_tons_hour: f64,  // 100.0 baseline
    pub replication_factor: u32,  // 2 conservative
    pub valence: f64,             // 0.0..=1.0
}

impl SwarmUnit {
    pub fn new(generation: u32) -> Self {
        SwarmUnit {
            generation,
            capacity_tons_hour: 100.0,
            replication_factor: 2,
            valence: 1.0,
        }
    }

    pub fn replicate(&self) -> Option<Vec<SwarmUnit>> {
        if self.valence >= 0.9999999 {
            let mut children = Vec::new();
            for _ in 0..self.replication_factor {
                children.push(SwarmUnit {
                    generation: self.generation + 1,
                    capacity_tons_hour: self.capacity_tons_hour * 1.1, // slight efficiency gain
                    replication_factor: self.replication_factor,
                    valence: self.valence,
                });
            }
            println!("Mercy-approved: Unit gen {} replicated → {} children", self.generation, children.len());
            Some(children)
        } else {
            println!("Mercy shield: Replication rejected (valence {:.7})", self.valence);
            None
        }
    }

    pub fn simulate_swarm_growth(&self, generations: u32) -> f64 {
        let mut total_units = 1.0;
        let mut current = self.clone();
        for _ in 0..generations {
            if let Some(children) = current.replicate() {
                total_units += children.len() as f64;
                current = children[0].clone(); // simulate lineage
            } else {
                break;
            }
        }
        total_units
    }
}

pub fn propose_swarm_scale(generations: u32) -> f64 {
    let seed = SwarmUnit::new(0);
    seed.simulate_swarm_growth(generations)
}
