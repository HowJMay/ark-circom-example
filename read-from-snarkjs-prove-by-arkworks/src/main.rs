use ark_bn254::{Bn254, Fq, Fq2, Fr as BnFr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_std::fs::File;
use ark_groth16::{Proof, VerifyingKey};
use serde_json::Value;
use std::io::{Read, Result as IoResult, Seek};
use ark_serialize::CanonicalSerialize;
use ark_circom::{read_zkey, CircomReduction, WitnessCalculator};
fn main() {
    let path = "./test-vectors/vanchor_circuit_final_2_2.zkey";
    let mut file = File::open(path).unwrap();
    let params = ark_circom::read_zkey(&mut file).unwrap();
    //let mut _wtns =
    // WitnessCalculator::new("./src/poseidon_vanchor_2_2.wasm").unwrap();
    // let mut _inputs: HashMap<String, Vec<num_bigint::BigInt>> = HashMap::new();
    let json = ark_std::fs::read_to_string("./test-vectors/proof.json").unwrap();
    let json: Value = serde_json::from_str(&json).unwrap();
    let proof = parse_proof_bn254_json(&json);

    let json = ark_std::fs::read_to_string("./test-vectors/inputs.json").unwrap();
    let json: Value = serde_json::from_str(&json).unwrap();
    let mut proof_serialized = Vec::new();
    ark_groth16::Proof::<Bn254>::serialize(&proof, &mut proof_serialized).unwrap();

    let mut pvk_serialized = Vec::new();
    VerifyingKey::<Bn254>::serialize(&params.vk, &mut pvk_serialized).unwrap();
    let inputs = arkworks_circom_verifier::verifier::parse_public_inputs_bn254_json(&json);
    //let verified = verify_proof(&pvk, &proof, &inputs).unwrap();
    let verified = arkworks_circom_verifier::verifier::verify(inputs, &pvk_serialized, &proof_serialized).unwrap();
    assert!(verified);
}

fn parse_proof_bn254_json(json: &Value) -> Proof<Bn254> {
	let pi_a = arkworks_circom_verifier::verifier::json_to_g1(json, "pi_a");
	let pi_b = arkworks_circom_verifier::verifier::json_to_g2(json, "pi_b");
	let pi_c = arkworks_circom_verifier::verifier::json_to_g1(json, "pi_c");

	Proof {
		a: pi_a,
		b: pi_b,
		c: pi_c,
	}
}

