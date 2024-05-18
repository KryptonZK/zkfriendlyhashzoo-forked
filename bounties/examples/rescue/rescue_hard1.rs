use ff::{from_hex, Field};
use zkhash_bounties::{
    fields::{field64::Fp64, utils},
    rescue_prime::{rescue_prime::RescuePrime, rescue_prime_instances::RESCUE_PRIME_PARAMS_HARD1},
};

type Scalar = Fp64;

static RANDOM_INPUT: bool = false;

fn main() {
    let params = &RESCUE_PRIME_PARAMS_HARD1;
    let rescue = RescuePrime::new(params);

    println!("Rescue Challange hard1");
    println!("N = {}", params.get_rounds());

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

    let output = rescue.permutation(&input);

    println!("Input  = {:?}", input);
    println!("Output = {:?}", output);

    if output[output.len() - 1] == Scalar::zero() {
        println!("Challenge solved!");
    } else {
        println!("Challenge not solved!");
    }
}
