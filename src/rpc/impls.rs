use ark_bn254::Bn254;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::Proof;
use ark_serialize::CanonicalDeserialize;
use std::fs::File;
use std::sync::mpsc;

use crate::rpc::api::ZgVcServer;
use crate::types::VcProof;

use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::types::error::INTERNAL_ERROR_CODE;
use jsonrpsee::core::async_trait;

use vc_prove::{
    groth16::verify,
    types::{ProveInput, VerifyInput},
};

pub struct RpcImpl {
    vk: PreparedVerifyingKey<Bn254>,
    sender: mpsc::Sender<(ProveInput, mpsc::Sender<Proof<Bn254>>)>,
}

impl RpcImpl {
    pub fn new(sender: mpsc::Sender<(ProveInput, mpsc::Sender<Proof<Bn254>>)>) -> Self {
        // load pk and vk from file
        let reader = File::open("output/check_vc.vk").expect("vk file should open success");
        let vk = PreparedVerifyingKey::<Bn254>::deserialize_uncompressed(reader)
            .expect("vk should load success");

        Self { vk, sender }
    }
}

#[async_trait]
impl ZgVcServer for RpcImpl {
    async fn generate_proof(&self, input: ProveInput) -> Result<VcProof, ErrorObjectOwned> {
        let (tx, rx) = mpsc::channel::<Proof<Bn254>>();
        self.sender
            .send((input, tx))
            .map_err(|e| jsonrpc_message_error(e.to_string()))?;
        let proof = rx
            .recv()
            .map_err(|e| jsonrpc_message_error(e.to_string()))?;
        Ok(VcProof(proof))
    }

    async fn verify_proof(&self, proof: VcProof, public_inputs: VerifyInput) -> Result<bool, ErrorObjectOwned> {
        let result = verify(&self.vk, &proof.0, &public_inputs)
            .map_err(|e| jsonrpc_message_error(e.to_string()))?;
        Ok(result)
    }

    async fn status(&self, input: String) -> Result<String, ErrorObjectOwned> {
        Ok(format!("status: {} is well!", input))
    }
}

pub fn jsonrpc_message_error(msg: String) -> ErrorObjectOwned {
	ErrorObjectOwned::owned(
		INTERNAL_ERROR_CODE,
		msg,
		Some(""),
	)
}
