// use openvm_stark_sdk::config::baby_bear_poseidon2_root::{BabyBearPoseidon2RootConfig, BabyBearPoseidon2RootEngine};
// use openvm_stark_sdk::config::FriParameters;
// use openvm_stark_sdk::engine::StarkFriEngine;
// use openvm_stark_sdk::utils::ProofInputForTest;
// use snark_verifier_sdk::Snark;
// use openvm_native_compiler::ir::Witness;
// use crate::config::outer::new_from_outer_multi_vk;
// use crate::gnark::GnarkProof;
// use crate::gnark::verifier::GnarkVerifierProvingKey;
// use crate::witness::Witnessable;
//
// pub fn run_static_verifier_test(
//     mut test_proof_input: ProofInputForTest<BabyBearPoseidon2RootConfig>,
//     fri_params: FriParameters,
// ) -> (GnarkVerifierProvingKey, GnarkProof) {
//     test_proof_input.sort_chips();
//     let info_span =
//         tracing::info_span!("prove outer stark to verify", step = "outer_stark_prove").entered();
//     let engine = BabyBearPoseidon2RootEngine::new(fri_params);
//     let vparams = test_proof_input.run_test(&engine).unwrap();
//
//     info_span.exit();
//
//     // Build verification program in eDSL.
//     let advice = new_from_outer_multi_vk(&vparams.data.vk);
//
//     let info_span = tracing::info_span!(
//         "keygen halo2 verifier circuit",
//         step = "static_verifier_keygen"
//     )
//         .entered();
//     let stark_verifier_circuit = generate_halo2_verifier_proving_key(
//         params,
//         advice,
//         &vparams.fri_params,
//         &vparams.data.proof,
//     );
//     info_span.exit();
//
//     let info_span = tracing::info_span!(
//         "prove halo2 verifier circuit",
//         step = "static_verifier_prove"
//     )
//         .entered();
//     let mut witness = Witness::default();
//     vparams.data.proof.write(&mut witness);
//     // let static_verifier_snark = stark_verifier_circuit.prove(params, witness, false);
//     info_span.exit();
//     // (stark_verifier_circuit, static_verifier_snark)
// }
