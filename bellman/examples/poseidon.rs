use bellman_ce::pairing::bls12_381::{self, Bls12};
use hash_r1cs::{
    circuits::Permutation,
    perm_groth::PermGroth,
    poseidon::{
        poseidon::Poseidon, poseidon_circuit::PoseidonCircuit,
        poseidon_instance_bls12::POSEIDON_BLS_3_PARAMS,
    },
    utils,
};
use rand::thread_rng;

type Scalar = bls12_381::Fr;

fn main() {
    println!("Poseidon proof (t = 3)");
    let poseidon = Poseidon::new(&POSEIDON_BLS_3_PARAMS);
    let poseidon_circuit = PoseidonCircuit::new(&POSEIDON_BLS_3_PARAMS);
    let mut rng = thread_rng();
    let mut groth = PermGroth::new(poseidon_circuit);
    println!("Create CRS");
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    println!("Sample input");
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    println!("Permutation");
    let perm = poseidon.permutation(&input);
    println!("Create proof");
    let proof = groth.create_proof(&input, &mut rng);

    println!("Verify proof");
    let result = PermGroth::<Bls12, PoseidonCircuit<Bls12>>::verify_proof(&pvk, &proof, &perm);

    match result {
        Ok(verified) => match verified {
            true => println!("Correct!"),
            false => println!("Proof was incorrect?"),
        },
        Err(_) => println!("Synthesis Error!?"),
    }
}
