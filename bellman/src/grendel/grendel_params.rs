use bellman_ce::pairing::{
    ff::{PrimeField, SqrtField},
    LegendreSymbol,
};

#[derive(Clone, Debug)]
pub struct GrendelParams<S: PrimeField + SqrtField> {
    pub(crate) t: usize, // statesize
    pub(crate) d: usize, // sbox degree
    pub(crate) rounds: usize,
    pub(crate) mds: Vec<Vec<S>>,
    pub(crate) round_constants: Vec<Vec<S>>,
    pub(crate) n: S,
}

impl<S: PrimeField + SqrtField> GrendelParams<S> {
    pub fn new(
        t: usize,
        d: usize,
        rounds: usize,
        mds: &[Vec<S>],
        round_constants: &[Vec<S>],
    ) -> Self {
        assert!(d == 2 || d == 3 || d == 5);
        assert_eq!(mds.len(), t);

        let mut n = S::zero();
        n.sub_assign(&S::one()); // n = -1

        if n.legendre() != LegendreSymbol::QuadraticNonResidue {
            n = S::one();
            while n.legendre() != LegendreSymbol::QuadraticNonResidue {
                n.add_assign(&S::one());
            }
        }

        GrendelParams {
            t,
            d,
            rounds,
            mds: mds.to_owned(),
            round_constants: round_constants.to_owned(),
            n,
        }
    }
}
