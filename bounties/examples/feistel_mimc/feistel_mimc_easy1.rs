use ff::{from_hex, Field};
use zkhash_bounties::{
    feistel_mimc::{feistel_mimc::FeistelMimc, feistel_mimc_instances::FM_PARAMS_EASY1},
    fields::{field64::Fp64, utils},
};

type Scalar = Fp64;

static RANDOM_INPUT: bool = false;

fn main() {
    let params = &FM_PARAMS_EASY1;
    let feistel_mimc = FeistelMimc::new(params);

    println!("FeistelMimc Challange easy1");
    println!("r = {}", params.get_rounds());

    // insert your solution here:
    let solution: Scalar = from_hex("0x0000000000000000").unwrap();

    let input = if RANDOM_INPUT {
        [utils::random_scalar(true), Scalar::zero()]
    } else {
        [solution, Scalar::zero()]
    };

    let output = feistel_mimc.permutation(&input);

    println!("Input  = {:?}", input);
    println!("Output = {:?}", output);

    if output[output.len() - 1] == Scalar::zero() {
        println!("Challenge solved!");
    } else {
        println!("Challenge not solved!");
    }
}
