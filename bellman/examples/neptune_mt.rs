use bellman_ce::pairing::bls12_381::{self, Bls12};
use hash_r1cs::{
    merkle_groth::MerkleGroth,
    merkle_tree::MerkleTree,
    mt_circuit::MerkleTreeCircuit,
    neptune::{
        neptune::Neptune, neptune_circuit::NeptuneCircuit, neptune_instances::NEPTUNE_BLS_4_PARAMS,
    },
    utils,
};
use rand::{
    distributions::{IndependentSample, Range},
    thread_rng,
};

type Scalar = bls12_381::Fr;

fn main() {
    println!("Neptune MT proof (2:1, 2^10 elements)");
    let perm = Neptune::new(&NEPTUNE_BLS_4_PARAMS);
    let mut mt = MerkleTree::new(perm);

    let log_set_size = 10;
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);

    let perm_circ = NeptuneCircuit::new(&NEPTUNE_BLS_4_PARAMS);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
    let mut groth = MerkleGroth::new(mt_circ);

    println!("Create CRS");
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    println!("Sample set");
    let set: Vec<Scalar> = (0..set_size)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();
    let index = dist.ind_sample(&mut rng);

    println!("Accumulate set");
    mt.accumulate(&set);

    println!("Create MT witness");
    let wit = mt.create_witness(&set[index]).unwrap();

    println!("Create proof");
    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    println!("Verify proof");
    let result = MerkleGroth::<Bls12, NeptuneCircuit<Bls12>>::verify_proof(
        &pvk,
        &proof,
        &mt.get_root().unwrap(),
    );

    match result {
        Ok(verified) => match verified {
            true => println!("Correct!"),
            false => println!("Proof was incorrect?"),
        },
        Err(_) => println!("Synthesis Error!?"),
    }
}
