use openvm_stark_backend::proof::Proof;
use openvm_stark_sdk::config::baby_bear_poseidon2_root::BabyBearPoseidon2RootConfig;
use openvm_stark_sdk::config::FriParameters;
use serde::{Deserialize, Serialize};
use openvm_native_compiler::ir::Witness;
use crate::config::outer::OuterConfig;
use crate::halo2::DslOperations;
use crate::stark::outer::build_circuit_verify_operations;
use crate::types::MultiStarkVerificationAdvice;
use crate::witness::Witnessable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnarkVerifierProvingKey {
    pub dsl_ops: DslOperations<OuterConfig>,

}

pub fn generate_gnark_verifier_proving_key(
    advice: MultiStarkVerificationAdvice<OuterConfig>,
    fri_params: &FriParameters,
    proof: &Proof<BabyBearPoseidon2RootConfig>,
) -> GnarkVerifierProvingKey {
    let mut witness = Witness::default();
    proof.write(&mut witness);
    let dsl_operations = build_circuit_verify_operations(advice, fri_params, proof);
    GnarkVerifierProvingKey {
        // pinning: Halo2Prover::keygen(params, dsl_operations.clone(), witness),
        dsl_ops: dsl_operations,
    }
}