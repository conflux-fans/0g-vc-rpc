use ark_bn254::Bn254;
use ark_circom::CircomBuilder;
use ark_groth16::{PreparedVerifyingKey, ProvingKey};
use ark_serialize::CanonicalDeserialize;
use jsonrpc_core::{Error, Result};
use std::fs::File;

use crate::rpc::api::ZgVc;
use crate::types::VcProof;

use vc_prove::{
    circuit::circom_builder,
    groth16::{
        prove,
        // setup,
        verify,
    },
    params::load_proving_key,
    types::{ProveInput, VerifyInput},
};

pub struct RpcImpl {
    pk: ProvingKey<Bn254>,
    circom: CircomBuilder<Bn254>,
    vk: PreparedVerifyingKey<Bn254>,
}

impl RpcImpl {
    pub fn new() -> Self {
        let circom = circom_builder(&"output".into(), "check_vc");
        // direct generate pk and vk
        // let (pk, vk) = setup(&circom);

        // load pk and vk from file
        let reader = File::open("output/check_vc.vk").unwrap();
        let vk = PreparedVerifyingKey::<Bn254>::deserialize_uncompressed(reader).unwrap();
        let pk = load_proving_key::<false>(&"output".into(), "check_vc").unwrap();

        Self { vk, pk, circom }
    }
}

impl ZgVc for RpcImpl {
    fn generate_proof(&self, input: ProveInput) -> Result<VcProof> {
        let proof = prove(&self.pk, &self.circom, input)
            .map_err(|e| Error::invalid_params(e.to_string()))?;
        Ok(VcProof(proof))
    }

    fn verify_proof(&self, proof: VcProof, public_inputs: VerifyInput) -> Result<bool> {
        let result = verify(&self.vk, &proof.0, &public_inputs)
            .map_err(|e| Error::invalid_params(e.to_string()))?;
        Ok(result)
    }
}
