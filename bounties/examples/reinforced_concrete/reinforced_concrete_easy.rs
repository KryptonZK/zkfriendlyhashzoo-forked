use ff::{from_hex, Field};
use zkhash_bounties::{
    fields::{field48::Fp48, utils},
    reinforced_concrete::{
        reinforced_concrete::ReinforcedConcrete, reinforced_concrete_instances::RC_PARAMS_EASY,
    },
};

type Scalar = Fp48;

static RANDOM_INPUT: bool = false;

fn main() {
    let params = &RC_PARAMS_EASY;
    let rc = ReinforcedConcrete::new(params);

    println!("Reinforced Concrete Challange easy");

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

    let output = rc.permutation(&input);

    println!("Input  = {:?}", input);
    println!("Output = {:?}", output);

    if output[output.len() - 1] == Scalar::zero() {
        println!("Challenge solved!");
    } else {
        println!("Challenge not solved!");
    }
}
