use bellman_ce::pairing::bls12_381::{self, Bls12};
use hash_r1cs::{
    circuits::Permutation,
    perm_groth::PermGroth,
    rescue::{
        rescue::Rescue, rescue_circuit::RescueCircuit, rescue_instance_bls12::RESCUE_BLS_3_PARAMS,
    },
    utils,
};
use rand::thread_rng;

type Scalar = bls12_381::Fr;

fn main() {
    println!("Rescue proof (t = 3)");
    let rescue = Rescue::new(&RESCUE_BLS_3_PARAMS);
    let rescue_circuit = RescueCircuit::new(&RESCUE_BLS_3_PARAMS);
    let mut rng = thread_rng();
    let mut groth = PermGroth::new(rescue_circuit);
    println!("Create CRS");
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    println!("Sample input");
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    println!("Permutation");
    let perm = rescue.permutation(&input);
    println!("Create proof");
    let proof = groth.create_proof(&input, &mut rng);

    println!("Verify proof");
    let result = PermGroth::<Bls12, RescueCircuit<Bls12>>::verify_proof(&pvk, &proof, &perm);

    match result {
        Ok(verified) => match verified {
            true => println!("Correct!"),
            false => println!("Proof was incorrect?"),
        },
        Err(_) => println!("Synthesis Error!?"),
    }
}
