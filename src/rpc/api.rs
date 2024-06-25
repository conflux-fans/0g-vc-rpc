use crate::types::{ProofWithMeta, VcBigInt, VcFr, VcProof};
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait ZgVc {
    #[rpc(name = "zg_generateZkProof")]
    fn generate_proof(
        &self,
        encoded_vc: String,
        birth_date_threshold: u64,
        path_elements: Vec<VcBigInt>,
        path_indices: Vec<u64>,
    ) -> Result<ProofWithMeta>;

    #[rpc(name = "zg_verifyZkProof")]
    fn verify_proof(&self, proof: VcProof, public_inputs: Vec<VcFr>) -> Result<bool>;
}
