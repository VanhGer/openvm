use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use openvm_stark_backend::p3_field::extension::BinomialExtensionField;
use openvm_stark_sdk::p3_baby_bear::BabyBear;
use openvm_stark_sdk::p3_bn254_fr::Bn254Fr;
use serde::{Deserialize, Serialize};
use openvm_native_compiler::constraints::Constraint;
use openvm_native_compiler::ir::{Config, Witness};
use crate::gnark::witness::GnarkWitness;

mod verifier;

#[cfg(test)]
mod tests;

pub mod testing_utils;
mod ffi;
mod witness;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GnarkProof {
    pub public_inputs: [String; 2],
    pub encoded_proof: String,
    pub raw_proof: String,
    pub groth16_vkey_hash: [u8; 32],
}


#[derive(Debug, Clone)]
pub struct GnarkProver;

impl GnarkProver {
    /// Creates a new [GnarkProver].
    pub fn new() -> Self {
        Self
    }

    /// Executes the prover in testing mode with a circuit definition and witness.
    pub fn test<C: Config>(constraints: Vec<Constraint>, witness: Witness<C>) {
        let serialized = serde_json::to_string(&constraints).unwrap();

        // Write constraints.
        let mut constraints_file = tempfile::NamedTempFile::new().unwrap();
        constraints_file.write_all(serialized.as_bytes()).unwrap();

        // Write witness.
        let mut witness_file = tempfile::NamedTempFile::new().unwrap();
        let gnark_witness = GnarkWitness::new(witness);
        let serialized = serde_json::to_string(&gnark_witness).unwrap();
        witness_file.write_all(serialized.as_bytes()).unwrap();

        // test_groth16_bn254(
        //     witness_file.path().to_str().unwrap(),
        //     constraints_file.path().to_str().unwrap(),
        // )
    }

    // pub fn build_contracts(build_dir: PathBuf) {
    //     // Write the corresponding asset files to the build dir.
    //     let sp1_verifier_path = build_dir.join("SP1VerifierGroth16.sol");
    //     let vkey_hash = Self::get_vkey_hash(&build_dir);
    //     let sp1_verifier_str = include_str!("../assets/SP1VerifierGroth16.txt")
    //         .replace("{SP1_CIRCUIT_VERSION}", SP1_CIRCUIT_VERSION)
    //         .replace("{VERIFIER_HASH}", format!("0x{}", hex::encode(vkey_hash)).as_str())
    //         .replace("{PROOF_SYSTEM}", "Groth16");
    //     fs::write(sp1_verifier_path, sp1_verifier_str).unwrap();
    // }

    /// Builds the Groth16 circuit locally.
    pub fn build<C: Config>(constraints: Vec<Constraint>, witness: Witness<C>, build_dir: PathBuf) {
        let serialized = serde_json::to_string(&constraints).unwrap();

        // Write constraints.
        let constraints_path = build_dir.join("constraints.json");
        let mut file = File::create(constraints_path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();

        // Write witness.
        let witness_path = build_dir.join("groth16_witness.json");
        let gnark_witness = GnarkWitness::new(witness);
        let mut file = File::create(witness_path).unwrap();
        let serialized = serde_json::to_string(&gnark_witness).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();

        // Build the circuit.
        // build_groth16_bn254(build_dir.to_str().unwrap());

        // Build the contracts.
        // Self::build_contracts(build_dir);
    }

    ///// Generates a Groth16 proof given a witness.
    // pub fn prove<C: Config>(&self, witness: Witness<C>, build_dir: PathBuf) -> Groth16Bn254Proof {
    //     // Write witness.
    //     let mut witness_file = tempfile::NamedTempFile::new().unwrap();
    //     let gnark_witness = GnarkWitness::new(witness);
    //     let serialized = serde_json::to_string(&gnark_witness).unwrap();
    //     witness_file.write_all(serialized.as_bytes()).unwrap();
    //
    //     // let mut proof =
    //     //     prove_groth16_bn254(build_dir.to_str().unwrap(), witness_file.path().to_str().unwrap());
    //     // proof.groth16_vkey_hash = Self::get_vkey_hash(&build_dir);
    //     // proof
    // }

    ///// Verify a Groth16proof and verify that the supplied vkey_hash and committed_values_digest
    ///// match.
    // pub fn verify(
    //     &self,
    //     proof: &Groth16Bn254Proof,
    //     vkey_hash: &BigUint,
    //     committed_values_digest: &BigUint,
    //     build_dir: &Path,
    // ) -> Result<()> {
    //     if proof.groth16_vkey_hash != Self::get_vkey_hash(build_dir) {
    //         return Err(anyhow::anyhow!(
    //             "Proof vkey hash does not match circuit vkey hash, it was generated with a different circuit."
    //         ));
    //     }
    //     verify_groth16_bn254(
    //         build_dir
    //             .to_str()
    //             .ok_or_else(|| anyhow::anyhow!("Failed to convert build dir to string"))?,
    //         &proof.raw_proof,
    //         &vkey_hash.to_string(),
    //         &committed_values_digest.to_string(),
    //     )
    //         .map_err(|e| anyhow::anyhow!("failed to verify proof: {}", e))
    // }
}

impl Default for GnarkProver {
    fn default() -> Self {
        Self::new()
    }
}