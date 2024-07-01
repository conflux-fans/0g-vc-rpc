use crate::types::VcProof;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use vc_prove::types::{ProveInput, VerifyInput};

#[rpc]
pub trait ZgVc {
    #[rpc(name = "zg_generateZkProof")]
    fn generate_proof(&self, input: ProveInput) -> Result<VcProof>;

    #[rpc(name = "zg_verifyZkProof")]
    fn verify_proof(&self, proof: VcProof, public_inputs: VerifyInput) -> Result<bool>;
}
