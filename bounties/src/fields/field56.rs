use ff::{Field, PrimeField, PrimeFieldRepr};

#[derive(PrimeField)]
#[PrimeFieldModulus = "72057594037926839"]
#[PrimeFieldGenerator = "11"]
pub struct Fp56(FrRepr);
