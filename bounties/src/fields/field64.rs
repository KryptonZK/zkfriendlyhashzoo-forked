use ff::{Field, PrimeField, PrimeFieldRepr};

#[derive(PrimeField)]
#[PrimeFieldModulus = "18446744073709551557"]
#[PrimeFieldGenerator = "2"]
pub struct Fp64(FrRepr);
