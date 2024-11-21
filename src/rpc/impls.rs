use ark_bn254::Bn254;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::Proof;
use ark_serialize::CanonicalDeserialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;

use crate::rpc::api::ZgVcServer;
use crate::types::VcProof;

use jsonrpsee::core::async_trait;
use jsonrpsee::types::error::INTERNAL_ERROR_CODE;
use jsonrpsee::types::ErrorObjectOwned;

use vc_prove::{
    groth16::verify,
    types::{VcProveInput, VcVerifyInput},
};

pub struct RpcImpl {
    vk: PreparedVerifyingKey<Bn254>,
    sender: mpsc::Sender<(VcProveInput, mpsc::Sender<Proof<Bn254>>)>,
}

impl RpcImpl {
    pub fn new(sender: mpsc::Sender<(VcProveInput, mpsc::Sender<Proof<Bn254>>)>) -> Self {
        // load pk and vk from file
        let reader = File::open("output/check_vc.vk").expect("vk file should open success");
        let vk = PreparedVerifyingKey::<Bn254>::deserialize_uncompressed(reader)
            .expect("vk should load success");

        Self { vk, sender }
    }
}

#[async_trait]
impl ZgVcServer for RpcImpl {
    async fn generate_proof(&self, input: VcProveInput) -> Result<VcProof, ErrorObjectOwned> {
        write_log(&format!(
            "generate_proof: {}\n",
            serde_json::to_string(&input).unwrap()
        ));

        let (tx, rx) = mpsc::channel::<Proof<Bn254>>();
        self.sender
            .send((input, tx))
            .map_err(|e| jsonrpc_message_error(e.to_string()))?;
        let proof = rx
            .recv()
            .map_err(|e| jsonrpc_message_error(e.to_string()))?;
        let res = VcProof(proof);

        write_log(&format!(
            "generate_proof result: {}\n",
            serde_json::to_string(&res).unwrap()
        ));

        Ok(res)
    }

    async fn verify_proof(
        &self,
        proof: VcProof,
        public_inputs: VcVerifyInput,
    ) -> Result<bool, ErrorObjectOwned> {
        write_log(&format!(
            "verify_proof: proof {} public_inputs {}\n",
            serde_json::to_string(&proof).unwrap(),
            serde_json::to_string(&public_inputs).unwrap()
        ));
        let result = verify(&self.vk, &proof.0, &public_inputs)
            .map_err(|e| jsonrpc_message_error(e.to_string()))?;

        write_log(&format!("verify_proof result: {}\n", result));
        Ok(result)
    }

    async fn echo(&self, input: String) -> Result<String, ErrorObjectOwned> {
        Ok(format!("Your input is: {}", input))
    }
}

pub fn jsonrpc_message_error(msg: String) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, msg, Some(""))
}

fn write_log(line: &str) {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open in append mode
        .open("nohup.log")
        .expect("Unable to open file");
    file.write_all(line.as_bytes())
        .expect("write log should success");
}
