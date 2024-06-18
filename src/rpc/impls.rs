use std::env;
use std::collections::HashMap;

use jsonrpc_core::Result;

use num_bigint::BigInt;
use ark_bn254::Fr;
use ark_std::rand::thread_rng;
use vc_prove::prove_backend::{cal_witness, gen_proof, ver_proof};

use crate::types::{VcFr, VcProof, VcProvingKey, GrothBn};
use crate::rpc::api::ZgVc;

pub struct RpcImpl;

impl ZgVc for RpcImpl {
    fn generate_proof(
        &self,
        encoded_vc: Vec<u8>,
        birth_date_threshold: u64,
        path_elements: Vec<u64>,
        path_indices: Vec<u64>,
    ) -> Result<(VcProof, VcProvingKey, Vec<VcFr>)> {
        let mut inputs = HashMap::new();
        inputs.insert(
            "encodedVC".to_string(),
            encoded_vc.iter().map(|&x| BigInt::from(x)).collect(),
        );
        inputs.insert(
            "birthDateThreshold".to_string(),
            vec![BigInt::from(birth_date_threshold)],
        );
        inputs.insert(
            "pathElements".to_string(),
            path_elements.iter().map(|&x| BigInt::from(x)).collect(),
        );
        inputs.insert(
            "pathIndices".to_string(),
            path_indices.iter().map(|&x| BigInt::from(x)).collect(),
        );

        let current_dir = env::current_dir().expect("Failed to get current directory");

        let (circuit, pub_in) = cal_witness(
            current_dir.join("output/check_vc_js/check_vc.wasm"),
            current_dir.join("output/check_vc.r1cs"),
            inputs,
        )
        .unwrap(); // TODO handle error

        let mut rng = thread_rng();
        let params =
            GrothBn::generate_random_parameters_with_reduction(circuit.clone(), &mut rng).unwrap();
        let proof = gen_proof(circuit, &params, &mut rng);

        let public_inputs: Vec<VcFr> = pub_in
            .into_iter()
            .map(|x| VcFr(x))
            .collect();

        Ok((VcProof(proof), VcProvingKey(params), public_inputs))
    }

    fn verify_proof(
        &self,
        pk: VcProvingKey,
        proof: VcProof,
        public_inputs: Vec<VcFr>,
    ) -> Result<bool> {
        let pi: Vec<Fr> = public_inputs.into_iter().map(|x| x.0).collect();
        let result = ver_proof(&pk.0, &proof.0, &pi);
        Ok(result)
    }
}


