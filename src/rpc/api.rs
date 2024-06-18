use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use crate::types::{VcFr, VcProof, VcProvingKey};

#[rpc]
pub trait ZgVc {
    #[rpc(name = "zg_generateZkProof")]
    fn generate_proof(
        &self,
        encoded_vc: Vec<u8>,
        birth_date_threshold: u64,
        path_elements: Vec<u64>,
        path_indices: Vec<u64>,
    ) -> Result<(VcProof, VcProvingKey, Vec<VcFr>)>;

    #[rpc(name = "zg_verifyZkProof")]
    fn verify_proof(
        &self,
        pk: VcProvingKey,
        proof: VcProof,
        public_inputs: Vec<VcFr>,
    ) -> Result<bool>;
}