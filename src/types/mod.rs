mod error;
mod vc_fr;
mod vc_proof;
mod vc_proving_key;

pub use vc_fr::VcFr;
pub use vc_proof::VcProof;
pub use vc_proving_key::VcProvingKey;

use ark_groth16::Groth16;
use ark_bn254::Bn254;

pub type GrothBn = Groth16<Bn254>;