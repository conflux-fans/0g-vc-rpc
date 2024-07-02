use ark_bn254::Bn254;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::Proof;
use ark_serialize::CanonicalDeserialize;
use jsonrpc_core::{Error, Result};
use std::fs::File;
use std::sync::mpsc;

use crate::rpc::api::ZgVc;
use crate::types::VcProof;

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

impl ZgVc for RpcImpl {
    fn generate_proof(&self, input: ProveInput) -> Result<VcProof> {
        let (tx, rx) = mpsc::channel::<Proof<Bn254>>();
        self.sender
            .send((input, tx))
            .map_err(|e| Error::invalid_params(e.to_string()))?;
        let proof = rx
            .recv()
            .map_err(|e| Error::invalid_params(e.to_string()))?;
        Ok(VcProof(proof))
    }

    fn verify_proof(&self, proof: VcProof, public_inputs: VerifyInput) -> Result<bool> {
        let result = verify(&self.vk, &proof.0, &public_inputs)
            .map_err(|e| Error::invalid_params(e.to_string()))?;
        Ok(result)
    }
}
