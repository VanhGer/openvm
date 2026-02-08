use std::{array, borrow::BorrowMut, sync::Arc};

use openvm_circuit::arch::{
    instructions::riscv::RV32_CELL_BITS,
    testing::{VmChipTestBuilder, BITWISE_OP_LOOKUP_BUS},
};
use openvm_circuit_primitives::{
    bitwise_op_lookup::{
        BitwiseOperationLookupAir, BitwiseOperationLookupBus, BitwiseOperationLookupChip,
        SharedBitwiseOperationLookupChip,
    },
    SubAir,
};
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    interaction::{BusIndex, InteractionBuilder},
    p3_air::{Air, BaseAir},
    p3_field::{Field, FieldAlgebra, PrimeField32},
    p3_matrix::{dense::RowMajorMatrix, Matrix},
    prover::{cpu::CpuBackend, types::AirProvingContext},
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
    utils::disable_debug_builder,
    verifier::VerificationError,
    AirRef, Chip,
};
use openvm_stark_sdk::{p3_baby_bear::BabyBear, utils::create_seeded_rng};
use openvm_stark_sdk::config::{setup_tracing, setup_tracing_with_log_level, FriParameters};
use openvm_stark_sdk::config::baby_bear_poseidon2::BabyBearPoseidon2Config;
use openvm_stark_sdk::config::baby_bear_poseidon2_root::BabyBearPoseidon2RootConfig;
use openvm_stark_sdk::utils::ProofInputForTest;
use rand::Rng;
use tracing_subscriber::filter::LevelFilter;
use openvm_circuit::arch::testing::TestSC;
use openvm_native_recursion::halo2::testing_utils::run_static_verifier_test;
use openvm_native_recursion::testing_utils::inner::{run_recursive_test};
use crate::{
    Sha256Air, Sha256DigestCols, Sha256FillerHelper, SHA256_BLOCK_U8S, SHA256_DIGEST_WIDTH,
    SHA256_HASH_WORDS, SHA256_WIDTH, SHA256_WORD_U8S,
};

// A wrapper AIR purely for testing purposes
#[derive(Clone, Debug)]
pub struct Sha256TestAir {
    pub sub_air: Sha256Air,
}

impl<F: Field> BaseAirWithPublicValues<F> for Sha256TestAir {}
impl<F: Field> PartitionedBaseAir<F> for Sha256TestAir {}
impl<F: Field> BaseAir<F> for Sha256TestAir {
    fn width(&self) -> usize {
        <Sha256Air as BaseAir<F>>::width(&self.sub_air)
    }
}

impl<AB: InteractionBuilder> Air<AB> for Sha256TestAir {
    fn eval(&self, builder: &mut AB) {
        self.sub_air.eval(builder, 0);
    }
}

const SELF_BUS_IDX: BusIndex = 28;
type F = BabyBear;
type RecordType = Vec<([u8; SHA256_BLOCK_U8S], bool)>;

// A wrapper Chip purely for testing purposes
pub struct Sha256TestChip {
    pub step: Sha256FillerHelper,
    pub bitwise_lookup_chip: SharedBitwiseOperationLookupChip<8>,
}

impl<SC: StarkGenericConfig> Chip<RecordType, CpuBackend<SC>> for Sha256TestChip
where
    Val<SC>: PrimeField32,
{
    fn generate_proving_ctx(&self, records: RecordType) -> AirProvingContext<CpuBackend<SC>> {
        let trace = crate::generate_trace::<Val<SC>>(
            &self.step,
            self.bitwise_lookup_chip.as_ref(),
            SHA256_WIDTH,
            records,
        );
        AirProvingContext::simple_no_pis(Arc::new(trace))
    }
}

#[allow(clippy::type_complexity)]
fn create_air_with_air_ctx<SC: StarkGenericConfig>() -> (
    (AirRef<SC>, AirProvingContext<CpuBackend<SC>>),
    (
        BitwiseOperationLookupAir<RV32_CELL_BITS>,
        SharedBitwiseOperationLookupChip<RV32_CELL_BITS>,
    ),
)
where
    Val<SC>: PrimeField32,
{
    let mut rng = create_seeded_rng();
    let bitwise_bus = BitwiseOperationLookupBus::new(BITWISE_OP_LOOKUP_BUS);
    let bitwise_chip = Arc::new(BitwiseOperationLookupChip::<RV32_CELL_BITS>::new(
        bitwise_bus,
    ));
    let len = 2;
    let random_records: Vec<_> = (0..len)
        .map(|i| {
            (
                array::from_fn(|_| rng.gen::<u8>()),
                rng.gen::<bool>() || i == len - 1,
            )
        })
        .collect();

    let air = Sha256TestAir {
        sub_air: Sha256Air::new(bitwise_bus, SELF_BUS_IDX),
    };
    let chip = Sha256TestChip {
        step: Sha256FillerHelper::new(),
        bitwise_lookup_chip: bitwise_chip.clone(),
    };
    let air_ctx = chip.generate_proving_ctx(random_records);

    ((Arc::new(air), air_ctx), (bitwise_chip.air, bitwise_chip))
}

#[test]
fn rand_sha256_test() {
    setup_tracing();
    // let tester = VmChipTestBuilder::default();
    let (air_ctx, bitwise) = create_air_with_air_ctx::<BabyBearPoseidon2RootConfig>();
    // let (air_ctx, bitwise) = create_air_with_air_ctx::<BabyBearPoseidon2Config>();
    // let tester = tester
    //     .build()
    //     .load_air_proving_ctx(air_ctx)
    //     .load_periphery(bitwise)
    //     .finalize();
    // tester.simple_test().expect("Verification failed");

    const LOG_BLOWUP: usize = 3;

    let proof = ProofInputForTest {
        airs: vec![air_ctx.0, Arc::new(bitwise.0)],
        per_air: vec![air_ctx.1, bitwise.1.generate_proving_ctx(())],
    };
    // let (_, proofs) = run_recursive_test_test(proof, FriParameters::new_for_testing(LOG_BLOWUP));
    // run_recursive_test(proof, FriParameters::new_for_testing(LOG_BLOWUP));
    run_static_verifier_test(proof, FriParameters::new_for_testing(LOG_BLOWUP));
}

#[test]
fn negative_sha256_test_bad_final_hash() {
    let tester = VmChipTestBuilder::default();
    let ((air, mut air_ctx), bitwise) = create_air_with_air_ctx();

    // Set the final_hash to all zeros
    let modify_trace = |trace: &mut RowMajorMatrix<F>| {
        trace.row_chunks_exact_mut(1).for_each(|row| {
            let mut row_slice = row.row_slice(0).to_vec();
            let cols: &mut Sha256DigestCols<F> = row_slice[..SHA256_DIGEST_WIDTH].borrow_mut();
            if cols.flags.is_last_block.is_one() && cols.flags.is_digest_row.is_one() {
                for i in 0..SHA256_HASH_WORDS {
                    for j in 0..SHA256_WORD_U8S {
                        cols.final_hash[i][j] = F::ZERO;
                    }
                }
                row.values.copy_from_slice(&row_slice);
            }
        });
    };

    // Modify the air_ctx
    let trace = Option::take(&mut air_ctx.common_main).unwrap();
    let mut trace = Arc::into_inner(trace).unwrap();
    modify_trace(&mut trace);
    air_ctx.common_main = Some(Arc::new(trace));

    disable_debug_builder();
    let tester = tester
        .build()
        .load_air_proving_ctx((air, air_ctx))
        .load_periphery(bitwise)
        .finalize();
    tester.simple_test_with_expected_error(VerificationError::OodEvaluationMismatch);
}
