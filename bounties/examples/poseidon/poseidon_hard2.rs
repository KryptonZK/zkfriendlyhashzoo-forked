use ff::{from_hex, Field};
use zkhash_bounties::{
    fields::{field64::Fp64, utils},
    poseidon::{poseidon::Poseidon, poseidon_instances::POSEIDON_PARAMS_HARD2},
};

type Scalar = Fp64;

static RANDOM_INPUT: bool = false;

fn main() {
    let params = &POSEIDON_PARAMS_HARD2;
    let poseidon = Poseidon::new(params);

    println!("Poseidon Challange hard2");
    println!("RP = {}", params.get_rp());

    // insert your solution here:
    let solution1: Scalar = from_hex("0x0000000000000000").unwrap();
    let solution2: Scalar = from_hex("0x0000000000000000").unwrap();

    let input = if RANDOM_INPUT {
        [
            utils::random_scalar(true),
            utils::random_scalar(true),
            Scalar::zero(),
        ]
    } else {
        [solution1, solution2, Scalar::zero()]
    };

    let output = poseidon.permutation(&input);

    println!("Input  = {:?}", input);
    println!("Output = {:?}", output);

    if output[output.len() - 1] == Scalar::zero() {
        println!("Challenge solved!");
    } else {
        println!("Challenge not solved!");
    }
}
