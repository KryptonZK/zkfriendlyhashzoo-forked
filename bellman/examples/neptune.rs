use bellman_ce::pairing::bls12_381::{self, Bls12};
use hash_r1cs::{
    circuits::Permutation,
    neptune::{
        neptune::Neptune, neptune_circuit::NeptuneCircuit, neptune_instances::NEPTUNE_BLS_4_PARAMS,
    },
    perm_groth::PermGroth,
    utils,
};
use rand::thread_rng;

type Scalar = bls12_381::Fr;

fn main() {
    println!("Neptune proof (t = 4)");
    let neptune = Neptune::new(&NEPTUNE_BLS_4_PARAMS);
    let neptune_circuit = NeptuneCircuit::new(&NEPTUNE_BLS_4_PARAMS);
    let mut rng = thread_rng();
    let mut groth = PermGroth::new(neptune_circuit);
    println!("Create CRS");
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    println!("Sample input");
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    println!("Permutation");
    let perm = neptune.permutation(&input);
    println!("Create proof");
    let proof = groth.create_proof(&input, &mut rng);

    println!("Verify proof");
    let result = PermGroth::<Bls12, NeptuneCircuit<Bls12>>::verify_proof(&pvk, &proof, &perm);

    match result {
        Ok(verified) => match verified {
            true => println!("Correct!"),
            false => println!("Proof was incorrect?"),
        },
        Err(_) => println!("Synthesis Error!?"),
    }
}
