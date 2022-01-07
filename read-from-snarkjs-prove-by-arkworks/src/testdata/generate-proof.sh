#!/bin/env bash

mkdir ./src/testdata/build
cd ./src/testdata/build

# 1. Start a new powers of tau ceremony
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
# 2. Contribute to the ceremony
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v
# 3. Provide a second contribution
snarkjs powersoftau contribute pot12_0001.ptau pot12_0002.ptau --name="Second contribution" -v -e="some random text"
# 4. Provide a third contribution using third party software
snarkjs powersoftau export challenge pot12_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot12_0002.ptau response_0003 pot12_0003.ptau -n="Third contribution name"
# 5. Verify the protocol so far
snarkjs powersoftau verify pot12_0003.ptau
# 6. Apply a random beacon
snarkjs powersoftau beacon pot12_0003.ptau pot12_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
# 7. Prepare phase 2
snarkjs powersoftau prepare phase2 pot12_beacon.ptau pot12_final.ptau -v
# 8. Verify the final ptau
snarkjs powersoftau verify pot12_final.ptau
# 10. Compile the circuit
circom ../circuit/circuit.circom --r1cs --wasm --sym
# 11. View information about the circuit
snarkjs r1cs info circuit.r1cs
# 12. Print the constraints
snarkjs r1cs print circuit.r1cs circuit.sym
# 13. Export r1cs to json
snarkjs r1cs export json circuit.r1cs circuit.r1cs.json
cat circuit.r1cs.json
# 14. Calculate the witness
cat <<EOT > input.json
{"a": 3, "b": 11}
EOT

cd circuit_js && node generate_witness.js circuit.wasm ../input.json ../witness.wtns && cd ..
# 15. Setup
snarkjs groth16 setup circuit.r1cs pot12_final.ptau circuit_0000.zkey
# 16. Contribute to the phase 2 ceremony
snarkjs zkey contribute circuit_0000.zkey circuit_0001.zkey --name="1st Contributor Name" -v
# 17. Provide a second contribution
snarkjs zkey contribute circuit_0001.zkey circuit_0002.zkey --name="Second contribution Name" -v -e="Another random entropy"
# 18. Provide a third contribution using third party software
snarkjs zkey export bellman circuit_0002.zkey  challenge_phase2_0003
snarkjs zkey bellman contribute bn128 challenge_phase2_0003 response_phase2_0003 -e="some random text"
snarkjs zkey import bellman circuit_0002.zkey response_phase2_0003 circuit_0003.zkey -n="Third contribution name"
# 19. Verify the latest zkey
snarkjs zkey verify circuit.r1cs pot12_final.ptau circuit_0003.zkey
# 20. Apply a random beacon
snarkjs zkey beacon circuit_0003.zkey circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
# 21. Verify the final zkey
snarkjs zkey verify circuit.r1cs pot12_final.ptau circuit_final.zkey
# 22. Export the verification key
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json
# 23. Create the proof
snarkjs groth16 prove circuit_final.zkey witness.wtns proof.json public.json
# 24. Verify the proof
snarkjs groth16 verify verification_key.json public.json proof.json
