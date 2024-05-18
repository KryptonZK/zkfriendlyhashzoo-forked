use zkhash::{
    fields::{st::FpST, utils},
    reinforced_concrete_st::{
        reinforced_concrete_st::ReinforcedConcreteSt,
        reinforced_concrete_st_instances::RC_ST_PARAMS,
    },
};
type Scalar = FpST;

fn perm_example() -> [Scalar; 3] {
    let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);

    println!("Sample input");
    let input: [Scalar; 3] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    println!("Permutation");
    rc.permutation(&input)
}

fn hash_example() -> Scalar {
    let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);

    println!("Sample input");
    let input1 = utils::random_scalar(true);
    let input2 = utils::random_scalar(true);

    println!("Hash");
    rc.hash(&input1, &input2)
}

fn main() {
    println!("ReinforcedConcrete ST Example");
    perm_example();
    hash_example();
}
