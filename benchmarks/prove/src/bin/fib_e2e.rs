use std::sync::Arc;
use clap::Parser;
use eyre::Result;
use openvm_stark_backend::config::StarkGenericConfig;
use openvm_stark_backend::engine::StarkEngine;
use openvm_benchmarks_prove::util::BenchmarkCli;
use openvm_circuit::arch::{VmCircuitConfig, DEFAULT_MAX_NUM_PUBLIC_VALUES};
use openvm_sdk::{config::SdkVmConfig, Sdk, StdIn};
use openvm_stark_sdk::bench::run_with_metric_collection;
use openvm_stark_sdk::config::baby_bear_poseidon2::BabyBearPoseidon2Engine;
use openvm_stark_sdk::engine::StarkFriEngine;
use openvm_circuit::system::program::trace::VmCommittedExe;
use openvm_continuations::SC;
use openvm_continuations::verifier::leaf::LeafVmVerifierConfig;
use openvm_sdk::prover::vm::types::VmProvingKey;
const NUM_PUBLIC_VALUES: usize = DEFAULT_MAX_NUM_PUBLIC_VALUES;

#[tokio::main]
async fn main() -> Result<()> {
    let args = BenchmarkCli::parse();

    // Must be larger than RangeTupleCheckerAir.height == 524288
    let max_segment_length = args.max_segment_length.unwrap_or(524288);

    let mut config =
        SdkVmConfig::from_toml(include_str!("../../../guest/fibonacci/openvm.toml"))?.app_vm_config;
    config
        .as_mut()
        .segmentation_limits
        .set_max_trace_height(max_segment_length);
    config.as_mut().num_public_values = NUM_PUBLIC_VALUES;

    let elf = args.build_bench_program("fibonacci", &config, None)?;
    let app_config = args.app_config(config);

    let sdk = Sdk::new(app_config)?;

    let n = 1000u64;
    // let mut stdin = StdIn::default();
    // stdin.write(&n);

    // sdk.execute()

    // let _proof = sdk.prover(elf)?.with_program_name("fib_e2e").prove(stdin)?;
    //
    // Ok(())

    run_with_metric_collection("OUTPUT_PATH", || -> eyre::Result<_> {
        // #[cfg(not(feature = "evm"))]
        // let mut prover = sdk.prover(elf)?.with_program_name("fib_e2e");
        // 
        // let root_proof = prover.generate_proof_for_outer_recursion(stdin)?;
        // 
        // let proof_size = bincode::serialize(&root_proof)?;
        // println!("Root proof size: {} bytes", proof_size.len());
        // 
        // 
        // for i in 0..root_proof.per_air.len() {
        //     let air_id = root_proof.per_air[i].air_id;
        //     let degree = root_proof.per_air[i].degree;
        //     println!("air: {}, degree: {}", air_id, degree);
        // }

        // let _proof = sdk.prover(elf)?.with_program_name("fib_e2e").prove(stdin)?;

        // let mut prover = sdk.app_prover(elf)?.with_program_name("fib_e2e");
        // let proof = prover.prove(stdin)?;

        // #[cfg(feature = "evm")]
        // let _proof = sdk
        //     .evm_prover(elf)?
        //     .with_program_name("fib_e2e")
        //     .prove_evm(stdin)?;

        let halo2_pk = sdk.halo2_pk();

        Ok(())
    })
}
