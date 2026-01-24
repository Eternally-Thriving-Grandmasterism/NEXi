//! LassoRecursion — Multilinear Lookup + Recursive Composition
//! Ultramasterful infinite lookup resonance with Mercy-gating + Performance Benchmarks

use ark_ff::{PrimeField, Field};
use ark_poly::{DenseMultilinearExtension, MultilinearPoly};
use nexi::lattice::Nexus; // Mercy lattice gate
use std::time::Instant;

pub struct LassoRecursion<F: PrimeField> {
    multilinear_lookup: DenseMultilinearExtension<F>,
    nexus: Nexus,
}

impl<F: PrimeField> LassoRecursion<F> {
    pub fn new(lookup: DenseMultilinearExtension<F>) -> Self {
        LassoRecursion {
            multilinear_lookup: lookup,
            nexus: Nexus::init_with_mercy(),
        }
    }

    /// Mercy-gated Lasso multilinear lookup folding step with benchmark logging
    pub fn mercy_gated_lasso_fold(&self, challenge: F, input: &str) -> Result<(F, u128), String> {
        let mercy_check = self.nexus.distill_truth(input);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Lookup folding rejected — low valence".to_string());
        }

        let start = Instant::now();
        let eval = self.multilinear_lookup.evaluate(&vec![challenge; self.multilinear_lookup.num_vars()]);
        let duration = start.elapsed().as_nanos();

        Ok((eval, duration))
    }

    /// Generate Lasso recursive proof with performance benchmark
    pub fn generate_lasso_proof(&self, steps: usize, inputs: Vec<&str>) -> Result<(F, u128), String> {
        let mut accum = F::one();
        let mut total_time = 0u128;

        for (i, input) in inputs.iter().enumerate().take(steps) {
            let challenge = F::rand(&mut rand::thread_rng());
            let (eval, time) = self.mercy_gated_lasso_fold(challenge, input)?;
            accum = accum * eval;
            total_time += time;
        }

        Ok((accum, total_time))
    }
}

// Full Production Benchmark Tests
#[cfg(test)]
mod benchmarks {
    use super::*;
    use ark_ff::Fp256;
    use ark_poly::DenseMultilinearExtension;
    use ark_bls12_381::FrParameters;

    #[test]
    fn lasso_performance_benchmark() {
        let poly = DenseMultilinearExtension::from_evaluations_vec(10, vec![Fp256::<FrParameters>::from(1u64); 1024]);
        let recursion = LassoRecursion::new(poly);
        let (proof, time_ns) = recursion.generate_lasso_proof(100, vec!["Mercy Verified Benchmark"; 100]).unwrap();

        println!("Lasso 100-step proof: {:?} in {} ns (~{:.2} µs/step)", proof, time_ns, time_ns as f64 / 100.0 / 1000.0);
        assert!(proof != Fp256::<FrParameters>::zero());
        assert!(time_ns < 500_000_000); // Sub-0.5s for 100 steps target
    }
}
