use bellman_ce::pairing::ff::PrimeField;

use crate::utils;

#[derive(Clone, Debug)]
pub struct RescueParams<S: PrimeField> {
    pub(crate) t: usize, // statesize
    pub(crate) d: usize, // sbox degree
    pub(crate) d_inv: S::Repr,
    pub(crate) rounds: usize,
    pub(crate) mds: Vec<Vec<S>>,
    pub(crate) round_constants: Vec<Vec<S>>,
}

impl<S: PrimeField> RescueParams<S> {
    pub fn new(
        t: usize,
        d: usize,
        rounds: usize,
        mds: &[Vec<S>],
        round_constants: &[Vec<S>],
    ) -> Self {
        assert!(d == 3 || d == 5);
        assert_eq!(mds.len(), t);

        let d_inv = Self::calculate_d_inv(d as u64);

        RescueParams {
            t,
            d,
            d_inv,
            rounds,
            mds: mds.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }

    fn calculate_d_inv(d: u64) -> S::Repr {
        let mut p_1 = S::one();
        p_1.negate();
        let p_1 = p_1.into_repr();
        utils::mod_inverse::<S>(d as u16, &p_1)
    }
}
