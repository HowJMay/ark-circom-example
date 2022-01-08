use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine, G1Projective, G2Projective};
use ark_ff::{BigInteger256};
use num::FromPrimitive;
use num_bigint::BigUint;

pub fn load_json_public_input(json: &serde_json::Value) -> Vec<Fr> {
	let elt: Vec<String> = serde_json::from_value(json.clone()).unwrap();

    let public_inputs = vec![str_to_fr(&elt[0])];
	public_inputs
}

pub fn load_json_verification_key(json: &serde_json::Value) -> ark_groth16::VerifyingKey::<Bn254> {
    ark_groth16::VerifyingKey::<Bn254> {
        alpha_g1: json_to_get_g1(&json, "vk_alpha_1"),
        beta_g2: json_to_get_g2(&json, "vk_beta_2"),
        gamma_g2: json_to_get_g2(&json, "vk_gamma_2"),
        delta_g2: json_to_get_g2(&json, "vk_delta_2"),
        gamma_abc_g1: json_to_get_g1_vec(&json, "IC"),
    }
}

pub fn load_json_proof(json: &serde_json::Value) -> ark_groth16::Proof<Bn254> {
	let pi_a = json_to_get_g1(json, "pi_a");
	let pi_b = json_to_get_g2(json, "pi_b");
	let pi_c = json_to_get_g1(json, "pi_c");

	ark_groth16::Proof {
		a: pi_a,
		b: pi_b,
		c: pi_c,
	}
}

fn json_to_get_g1(json: &serde_json::Value, key: &str) -> G1Affine {
    let els: Vec<String> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i.as_str().unwrap().to_string())
        .collect();
    G1Affine::from(G1Projective::new(
        str_to_fq(&els[0]),
        str_to_fq(&els[1]),
        str_to_fq(&els[2]),
    ))
}

fn json_to_get_g1_vec(json: &serde_json::Value, key: &str) -> Vec<G1Affine> {
    let els: Vec<Vec<String>> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            i.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    els.iter()
        .map(|coords| {
            G1Affine::from(G1Projective::new(
                str_to_fq(&coords[0]),
                str_to_fq(&coords[1]),
                str_to_fq(&coords[2]),
            ))
        })
        .collect()
}

fn json_to_get_g2(json: &serde_json::Value, key: &str) -> G2Affine {
    let els: Vec<Vec<String>> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            i.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let x = Fq2::new(str_to_fq(&els[0][0]), str_to_fq(&els[0][1]));
    let y = Fq2::new(str_to_fq(&els[1][0]), str_to_fq(&els[1][1]));
    let z = Fq2::new(str_to_fq(&els[2][0]), str_to_fq(&els[2][1]));
    G2Affine::from(G2Projective::new(x, y, z))
}

use std::str::FromStr;
use std::convert::TryFrom;

pub fn str_to_fq(s: &str) -> Fq {
    BigInteger256::try_from(BigUint::from_str(s).unwrap())
        .unwrap()
        .into()
}

pub fn int_to_fr(n: i64) -> Fr {
	BigInteger256::try_from(BigUint::from_i64(n).unwrap())
		.unwrap()
		.into()
}

pub fn str_to_fr(s: &str) -> Fr {
	BigInteger256::try_from(BigUint::from_str(s).unwrap())
		.unwrap()
		.into()
}

fn json_to_get_fr(json: &serde_json::Value, key: &str) -> Fr {	
    let els = json.get(key).unwrap();

	int_to_fr(els.as_i64().unwrap())
}
