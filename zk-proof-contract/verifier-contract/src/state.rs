use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use bellman::groth16::{ Proof, VerifyingKey, PreparedVerifyingKey };
use bellman::groth16::prepare_verifying_key;
use bls12_381::{ G1Affine, G2Affine, Bls12, Scalar };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("state");

// ZK

#[derive(Serialize, Deserialize)]
pub struct VkeyStr {
    pub alpha_1: Vec<u8>,
    pub beta_2: Vec<u8>,
    pub gamma_2: Vec<u8>,
    pub delta_2: Vec<u8>,
    pub ic: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct ProofStr {
    pub pi_a: Vec<u8>,
    pub pi_b: Vec<u8>,
    pub pi_c: Vec<u8>,
}

pub fn get_verification_key() -> PreparedVerifyingKey<Bls12> {
    let verification_key_str = r#"{"alpha_1":[1,15,125,182,194,214,187,59,39,41,185,168,217,237,64,22,149,168,240,60,27,201,60,209,10,184,197,249,201,249,173,24,239,239,7,150,4,180,31,176,206,34,78,89,208,178,3,234,16,128,179,155,175,112,85,150,16,140,242,9,235,195,22,85,249,105,74,55,203,13,72,253,108,144,91,173,111,132,239,26,1,42,155,238,96,88,141,188,175,223,68,207,124,176,179,185],"beta_2":[19,242,66,155,85,17,175,139,133,247,63,90,218,163,168,15,12,94,126,20,245,87,119,233,124,15,220,216,1,156,215,103,136,65,115,8,211,134,46,235,17,166,10,227,169,42,209,92,24,192,114,79,138,129,87,52,251,36,122,61,114,134,82,78,25,124,186,204,116,151,242,156,206,181,63,56,223,5,235,185,64,145,5,107,59,227,178,203,62,4,72,100,91,168,157,84,24,42,57,197,81,17,156,57,84,231,196,13,106,10,38,112,36,17,10,40,92,30,86,42,133,112,132,7,99,87,38,138,44,101,59,249,74,249,57,136,160,250,59,105,243,29,220,141,18,244,64,16,174,170,73,225,146,233,210,156,139,135,64,179,180,52,8,15,212,255,62,161,185,173,103,55,188,142,250,145,50,222,54,51,7,5,254,49,86,176,2,130,200,61,240,93],"gamma_2":[19,224,43,96,82,113,159,96,125,172,211,160,136,39,79,101,89,107,208,208,153,32,182,26,181,218,97,187,220,127,80,73,51,76,241,18,19,148,93,87,229,172,125,5,93,4,43,126,2,74,162,178,240,143,10,145,38,8,5,39,45,197,16,81,198,228,122,212,250,64,59,2,180,81,11,100,122,227,209,119,11,172,3,38,168,5,187,239,212,128,86,200,193,33,189,184,6,6,196,160,46,167,52,204,50,172,210,176,43,194,139,153,203,62,40,126,133,167,99,175,38,116,146,171,87,46,153,171,63,55,13,39,92,236,29,161,170,169,7,95,240,95,121,190,12,229,213,39,114,125,110,17,140,201,205,198,218,46,53,26,173,253,155,170,140,189,211,167,109,66,154,105,81,96,209,44,146,58,201,204,59,172,162,137,225,147,84,134,8,184,40,1],"delta_2":[12,100,203,106,241,155,89,207,240,32,103,85,204,197,194,89,236,140,37,204,118,111,204,46,26,44,69,231,21,219,20,102,244,76,177,68,14,234,102,212,251,250,248,101,103,79,246,177,4,22,116,11,192,166,220,221,201,38,163,127,95,206,249,85,249,32,67,57,181,93,121,18,74,121,228,128,55,124,156,54,133,242,193,186,181,18,63,203,214,137,102,197,252,171,185,10,6,111,24,252,154,101,65,249,76,236,22,57,176,129,9,56,241,70,46,253,192,98,57,240,12,245,22,88,178,57,94,188,84,181,145,210,193,232,183,50,54,116,145,205,54,253,244,100,12,63,1,136,66,21,37,50,39,245,169,214,230,251,90,124,192,185,78,65,146,172,16,155,197,242,31,37,2,140,182,141,203,99,205,88,140,239,176,191,136,50,76,181,36,100,10,38],"ic":[[25,130,226,19,123,128,241,61,98,49,149,190,132,254,121,57,25,40,172,200,84,231,203,47,34,188,170,135,142,0,106,157,241,28,236,108,191,42,209,234,86,148,123,86,220,112,20,150,16,161,170,190,217,103,89,115,82,168,129,43,152,171,220,117,85,149,106,196,208,163,118,148,158,132,131,79,74,73,168,76,15,209,176,104,85,177,196,41,137,220,80,215,202,164,183,230],[2,46,19,189,33,253,118,197,231,75,146,201,191,130,15,84,45,216,210,111,132,162,30,90,239,243,42,220,63,214,184,43,177,213,136,53,21,47,74,102,5,95,133,237,140,187,82,20,5,234,156,212,250,184,215,251,0,137,26,8,81,242,131,54,22,223,183,100,41,137,48,18,17,160,138,238,15,22,228,244,206,2,205,183,0,66,230,62,36,140,241,183,34,78,189,250]]}"#;
    let v_key = verification_key_str.to_string();
    let vk: VkeyStr = serde_json::from_str(&v_key).unwrap();
    let vk_alpha_1 = vk.alpha_1;
    let vk_beta_2 = vk.beta_2;
    let vk_gamma_2 = vk.gamma_2;
    let vk_delta_2 = vk.delta_2;
    let vk_ic = vk.ic;
    
    let mut alpha1: [u8; 96] = [0; 96];
    let mut beta2: [u8; 192] = [0; 192];
    let mut gamma2: [u8; 192] = [0; 192];
    let mut delta2: [u8; 192] = [0; 192];
    let mut ic_0: [u8; 96] = [0; 96];
    let mut ic_1: [u8; 96] = [0; 96];
    let mut ic = Vec::new();

    for i in 0..vk_alpha_1.len() {
        alpha1[i] = vk_alpha_1[i];
    }

    for i in 0..vk_beta_2.len() {
        beta2[i] = vk_beta_2[i];
    }

    for i in 0..vk_gamma_2.len() {
        gamma2[i] = vk_gamma_2[i];
    }

    for i in 0..vk_delta_2.len() {
        delta2[i] = vk_delta_2[i];
    }

    for i in 0..vk_ic[0].len() {
        ic_0[i] = vk_ic[0][i];
    }

    for i in 0..vk_ic[1].len() {
        ic_1[i] = vk_ic[1][i];
    }
    let alpha1_affine = G1Affine::from_uncompressed(&alpha1).unwrap();
    let beta2_affine = G2Affine::from_uncompressed(&beta2).unwrap();
    let gamma2_affine = G2Affine::from_uncompressed(&gamma2).unwrap();
    let delta2_affine = G2Affine::from_uncompressed(&delta2).unwrap();
    let ic0_affine = G1Affine::from_uncompressed(&ic_0).unwrap();
    let ic1_affine = G1Affine::from_uncompressed(&ic_1).unwrap();
    ic.push(ic0_affine);
    ic.push(ic1_affine);

    let vkk: VerifyingKey<Bls12> = VerifyingKey{
        alpha_g1: alpha1_affine,
        beta_g1: G1Affine::identity(),
        beta_g2: beta2_affine,
        gamma_g2: gamma2_affine,
        delta_g1: G1Affine::identity(),
        delta_g2: delta2_affine,
        ic,
    };
    prepare_verifying_key(&vkk)
}

pub fn parse_proof(proof: String) -> Proof<Bls12> {
    let pof: ProofStr = serde_json::from_str(&proof).unwrap();
    let pi_a = pof.pi_a;
    let pi_b = pof.pi_b;
    let pi_c = pof.pi_c;
    let mut a_arr: [u8; 96] = [0; 96];
    let mut b_arr: [u8; 192] = [0; 192];
    let mut c_arr: [u8; 96] = [0; 96];
    for i in 0..pi_a.len() {
        a_arr[i] = pi_a[i];
    }

    for i in 0..pi_b.len() {
        b_arr[i] = pi_b[i];
    }

    for i in 0..pi_c.len() {
        c_arr[i] = pi_c[i];
    }
    let pia_affine = G1Affine::from_uncompressed(&a_arr).unwrap();
    let pib_affine = G2Affine::from_uncompressed(&b_arr).unwrap();
    let pic_affine = G1Affine::from_uncompressed(&c_arr).unwrap();
    let proof: Proof<Bls12> = Proof {
        a: pia_affine,
        b: pib_affine,
        c: pic_affine,
    };
    proof
}

pub fn get_public_signal(value: u64) -> Vec<Scalar> {
    vec![Scalar::from(value)]
}
