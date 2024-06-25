mod error;
mod proof_with_meta;
mod vc_bigint;
mod vc_fr;
mod vc_proof;
mod vc_proving_key;

pub use proof_with_meta::ProofWithMeta;
pub use vc_bigint::VcBigInt;
pub use vc_fr::VcFr;
pub use vc_proof::VcProof;
// pub use vc_proving_key::VcProvingKey;

use ark_bn254::Bn254;
use ark_groth16::Groth16;

pub type GrothBn = Groth16<Bn254>;
