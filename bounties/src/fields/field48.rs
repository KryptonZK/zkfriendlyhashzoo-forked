use ff::{Field, PrimeField, PrimeFieldRepr};

#[derive(PrimeField)]
#[PrimeFieldModulus = "281474976710597"]
#[PrimeFieldGenerator = "2"]
pub struct Fp48(FrRepr);
