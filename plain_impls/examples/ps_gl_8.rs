use std::env;
use std::fs;
use ff::hex::FromHex;
use zkhash::{
    fields::{f64::F64, utils},
    poseidon::{poseidon::Poseidon, poseidon_instance_goldilocks::POSEIDON_GOLDILOCKS_8_PARAMS,
    },
};
type Scalar = F64;

fn perm_example(seed: &[usize]) {

    let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_8_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar_with_seed(true, seed)).collect();
    poseidon.permutation(&input);

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
    
    let seed: &[usize] = unsafe {std::slice::from_raw_parts(u8_array.as_ptr() as *const usize, 16)};

    println!("Poseidon GL 8 Example");
    perm_example(seed);
}