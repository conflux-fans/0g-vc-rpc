use crate::types::VcProof;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::ErrorObjectOwned;
use vc_prove::types::{ProveInput, VerifyInput};

#[rpc(server, namespace = "zg")]
pub trait ZgVc {
    #[method(name = "generateZkProof")]
    async fn generate_proof(&self, input: ProveInput) -> Result<VcProof, ErrorObjectOwned>;

    #[method(name = "verifyZkProof")]
    async fn verify_proof(
        &self,
        proof: VcProof,
        public_inputs: VerifyInput,
    ) -> Result<bool, ErrorObjectOwned>;

    #[method(name = "echo")]
    async fn echo(&self, input: String) -> Result<String, ErrorObjectOwned>;
}
