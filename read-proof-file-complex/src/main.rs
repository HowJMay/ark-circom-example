use ark_groth16::{
    prepare_verifying_key, verify_proof,
};
use std::path::Path;
mod loader;

fn main() {
    let public_path = Path::new("./src/testdata/public.json");
    let verification_key_path = Path::new("./src/testdata/verification_key.json");
    let proof_path = Path::new("./src/testdata/proof.json");
    
    let json = std::fs::read_to_string(public_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let inputs = loader::load_json_public_input(&json);
    
    let json = std::fs::read_to_string(verification_key_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let vk= loader::load_json_verification_key(&json);
    let pvk = prepare_verifying_key(&vk);

    let json = ark_std::fs::read_to_string(proof_path).unwrap();
	let json: serde_json::Value = serde_json::from_str(&json).unwrap();
	let proof = loader::load_json_proof(&json);

    let verified = verify_proof(&pvk, &proof, &inputs).unwrap();
    assert!(verified);
    print!("verified: {}\n", verified);
}
