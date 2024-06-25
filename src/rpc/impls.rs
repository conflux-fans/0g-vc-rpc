use std::collections::HashMap;
use std::env;

use ark_bn254::{Bn254, Fr};
use ark_groth16::ProvingKey;
use ark_std::rand::thread_rng;
use const_hex::decode;
use jsonrpc_core::{Error, Result};
use num_bigint::BigInt;
use vc_prove::prove_backend::{cal_witness, gen_proof, ver_proof};
// use std::sync::{Arc, RwLock};
// use rand::rngs::ThreadRng;

use crate::rpc::api::ZgVc;
use crate::types::{GrothBn, ProofWithMeta, VcBigInt, VcFr, VcProof};

use ark_circom::CircomCircuit;

pub struct RpcImpl {
    // circuit: CircomCircuit<Bn254>,
    // rng: ThreadRng,
    params: ProvingKey<Bn254>,
}

impl RpcImpl {
    pub fn new() -> Self {
        let circuit = get_circuit(HashMap::new()).unwrap().0;
        let mut rng = thread_rng();
        let params =
            GrothBn::generate_random_parameters_with_reduction(circuit.clone(), &mut rng).unwrap();

        Self {
            // circuit,
            params,
        }
    }
}

impl ZgVc for RpcImpl {
    fn generate_proof(
        &self,
        encoded_vc: String,
        birth_date_threshold: u64,
        path_elements: Vec<VcBigInt>,
        path_indices: Vec<u64>,
    ) -> Result<ProofWithMeta> {
        let encoded_vc = decode(encoded_vc)
            .map_err(|e| Error::invalid_params(format!("encoded_vc invalid {e}")))?;
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
            path_elements.iter().map(|x| x.0.clone()).collect(),
        );
        inputs.insert(
            "pathIndices".to_string(),
            path_indices.iter().map(|&x| BigInt::from(x)).collect(),
        );

        let (circuit, pub_in) = get_circuit(inputs)?;

        let mut rng = thread_rng();
        let proof = gen_proof(circuit, &self.params, &mut rng);

        let public_inputs: Vec<VcFr> = pub_in.into_iter().map(|x| VcFr(x)).collect();

        Ok(ProofWithMeta {
            proof: VcProof(proof),
            public_inputs,
        })
    }

    fn verify_proof(&self, proof: VcProof, public_inputs: Vec<VcFr>) -> Result<bool> {
        let pi: Vec<Fr> = public_inputs.into_iter().map(|x| x.0).collect();
        let result = ver_proof(&self.params, &proof.0, &pi);
        Ok(result)
    }
}

fn get_circuit(inputs: HashMap<String, Vec<BigInt>>) -> Result<(CircomCircuit<Bn254>, Vec<Fr>)> {
    let current_dir = env::current_dir().map_err(|_e| Error::internal_error())?;

    let (circuit, pub_in) = cal_witness(
        current_dir.join("output/check_vc_js/check_vc.wasm"),
        current_dir.join("output/check_vc.r1cs"),
        inputs,
    )
    .map_err(|e| Error::invalid_params(e.to_string()))?;

    Ok((circuit, pub_in))
}
