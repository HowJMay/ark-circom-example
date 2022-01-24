use std::path::Path;
mod utils {
    pub mod loader;
    pub mod verifier;
    pub mod data_structures;
}
mod testdata_json;

fn main() {
    let testdata = testdata_json::Json::new();
    
    let json: serde_json::Value = serde_json::from_str(&testdata.public).unwrap();
    let inputs = utils::loader::load_json_public_input(&json);
    
    let json: serde_json::Value = serde_json::from_str(&testdata.verification_key).unwrap();
    let vk= utils::loader::load_json_verification_key(&json);
    let pvk = utils::verifier::prepare_verifying_key(&vk);

	let json: serde_json::Value = serde_json::from_str(&testdata.proof).unwrap();
	let proof = utils::loader::load_json_proof(&json);

    let verified = utils::verifier::verify_proof(&pvk, &proof, &inputs).unwrap();
    assert!(verified);
    print!("verified: {}\n", verified);
}
