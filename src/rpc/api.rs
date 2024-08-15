use crate::types::VcProof;
use vc_prove::types::{ProveInput, VerifyInput};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::ErrorObjectOwned;

#[rpc(client, server, namespace = "zg")]
pub trait ZgVc {
    #[method(name = "generateZkProof")]
    async fn generate_proof(&self, input: ProveInput) -> Result<VcProof, ErrorObjectOwned>;

    #[method(name = "verifyZkProof")]
    async fn verify_proof(&self, proof: VcProof, public_inputs: VerifyInput) -> Result<bool, ErrorObjectOwned>;

    #[method(name = "status")]
    async fn status(&self, input: String) -> Result<String, ErrorObjectOwned>;
}
