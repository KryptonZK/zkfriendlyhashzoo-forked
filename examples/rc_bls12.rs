use ff::hex::FromHex;
use std::env;
use std::fs;
use zkhash::{
    fields::{bls12::FpBLS12, utils},
    reinforced_concrete::{
        reinforced_concrete::ReinforcedConcrete, reinforced_concrete_instances::RC_BLS_PARAMS,
    },
};
type Scalar = FpBLS12;

fn perm_example(seed: &[usize]) -> [Scalar; 3] {
    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);

    println!("Sample input");
    let input: [Scalar; 3] = [
        utils::random_scalar_with_seed(true, seed),
        utils::random_scalar_with_seed(true, seed),
        utils::random_scalar_with_seed(true, seed),
    ];

    println!("Permutation");
    rc.permutation(&input)
}

fn hash_example(seed: &[usize]) -> Scalar {
    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);

    println!("Sample input");
    let input1 = utils::random_scalar_with_seed(true, seed);
    let input2 = utils::random_scalar_with_seed(true, seed);

    println!("Hash");
    rc.hash(&input1, &input2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program name: {}", args[0]);

    // If there are additional arguments, print them
    if args.len() > 1 {
        println!("Arguments:");
        for arg in args.iter().skip(1) {
            println!("{}", arg);
        }
    } else {
        println!("No arguments provided.");
    }

    let file_contents = match fs::read_to_string(args[1].as_str()) {
        Ok(contents) => contents,
        Err(e) => {
            // If there was an error, print the error message and exit the program
            println!("Error reading file: {}", e);
            return;
        }
    };
    println!("Key: {}", file_contents);

    let u8_array = match Vec::<u8>::from_hex(file_contents) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let seed: &[usize] =
        unsafe { std::slice::from_raw_parts(u8_array.as_ptr() as *const usize, 16) };

    println!("ReinforcedConcrete BLS12 Example");
    perm_example(seed);
    hash_example(seed);
}
