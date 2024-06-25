use super::{VcFr, VcProof};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProofWithMeta {
    pub proof: VcProof,
    pub public_inputs: Vec<VcFr>,
}
