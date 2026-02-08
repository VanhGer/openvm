use crate::config::outer::{new_from_outer_multi_vk, OuterConfig};
use crate::gnark::GnarkProver;
use crate::stark::outer::{build_circuit_verify_constraints, build_circuit_verify_operations};
use crate::witness::Witnessable;
use openvm_native_compiler::constraints::ConstraintCompiler;
use openvm_native_compiler::ir::Witness;
use openvm_stark_sdk::config::baby_bear_poseidon2_root::{
    BabyBearPoseidon2RootConfig, BabyBearPoseidon2RootEngine,
};
use openvm_stark_sdk::config::{setup_tracing_with_log_level, FriParameters};
use openvm_stark_sdk::engine::StarkFriEngine;
use openvm_stark_sdk::utils::ProofInputForTest;
use tracing::Level;

use crate::tests::{fibonacci_test_proof_input, interaction_test_proof_input};

#[test]
fn test_fibonacci_gnark() {
    crate::gnark::tests::stark::run_recursive_test(fibonacci_test_proof_input::<
        BabyBearPoseidon2RootConfig,
    >(16))
}

fn run_recursive_test(mut test_proof_input: ProofInputForTest<BabyBearPoseidon2RootConfig>) {
    setup_tracing_with_log_level(Level::WARN);
    test_proof_input.sort_chips();
    let vparams = test_proof_input
        .run_test(&BabyBearPoseidon2RootEngine::new(
            FriParameters::standard_fast(),
        ))
        .unwrap();
    let advice = new_from_outer_multi_vk(&vparams.data.vk);
    let proof = vparams.data.proof;

    let mut witness = Witness::default();
    proof.write(&mut witness);
    let constraints = build_circuit_verify_constraints(advice, &vparams.fri_params, &proof);

    println!("{:?}", constraints);
    println!("{:?}", witness);
    GnarkProver::test(constraints, witness);
}
