use core::panic;
use std::sync::Arc;

use ff::PrimeField;

use super::feistel_mimc_params::FeistelMimcParams;

#[derive(Clone, Debug)]
pub struct FeistelMimc<F: PrimeField> {
    pub(crate) params: Arc<FeistelMimcParams<F>>,
}

impl<F: PrimeField> FeistelMimc<F> {
    pub fn new(params: &Arc<FeistelMimcParams<F>>) -> Self {
        FeistelMimc {
            params: Arc::clone(params),
        }
    }

    fn sbox(&self, state_0: &F, round: usize) -> F {
        let mut input = *state_0;
        input.add_assign(&self.params.round_constants[round]);

        let mut input2 = input.to_owned();
        input2.square();
        match self.params.d {
            3 => {}
            5 => input2.square(),
            _ => panic!(),
        }
        input2.mul_assign(&input);
        input2
    }

    fn round(&self, state: &mut [F; 2], round: usize) {
        let power = self.sbox(&state[0], round);
        state[1].add_assign(&power);
    }

    pub fn permutation(&self, input: &[F; 2]) -> [F; 2] {
        let mut current_state = input.to_owned();
        for r in 0..self.params.rounds - 1 {
            self.round(&mut current_state, r);
            current_state.swap(0, 1);
        }

        // finally without rotation
        self.round(&mut current_state, self.params.rounds - 1);

        current_state
    }
}

// #############################################################################

#[cfg(test)]
mod feistel_mimc_kats {
    use super::*;

    use crate::feistel_mimc::feistel_mimc_instances::*;
    use crate::fields::field64::Fp64;

    use ff::{from_hex, Field};

    type Scalar = Fp64;

    #[test]
    fn easy1_kats() {
        let fm = FeistelMimc::new(&FM_PARAMS_EASY1);
        let input: [Scalar; 2] = [Scalar::zero(), Scalar::one()];
        let perm = fm.permutation(&input);
        assert_eq!(perm[0], from_hex("0xf874e35bbaf92376").unwrap());
        assert_eq!(perm[1], from_hex("0x72af6f65901ac3f1").unwrap());
    }

    #[test]
    fn easy2_kats() {
        let fm = FeistelMimc::new(&FM_PARAMS_EASY2);
        let input: [Scalar; 2] = [Scalar::zero(), Scalar::one()];
        let perm = fm.permutation(&input);
        assert_eq!(perm[0], from_hex("0x85bd8eb1f92bfb9a").unwrap());
        assert_eq!(perm[1], from_hex("0x49d9875c885a962c").unwrap());
    }

    #[test]
    fn medium_kats() {
        let fm = FeistelMimc::new(&FM_PARAMS_MEDIUM);
        let input: [Scalar; 2] = [Scalar::zero(), Scalar::one()];
        let perm = fm.permutation(&input);
        assert_eq!(perm[0], from_hex("0x6f069da7d13eeac0").unwrap());
        assert_eq!(perm[1], from_hex("0xf99209102b0f4e3b").unwrap());
    }

    #[test]
    fn hard1_kats() {
        let fm = FeistelMimc::new(&FM_PARAMS_HARD1);
        let input: [Scalar; 2] = [Scalar::zero(), Scalar::one()];
        let perm = fm.permutation(&input);
        assert_eq!(perm[0], from_hex("0x1017818eae881aee").unwrap());
        assert_eq!(perm[1], from_hex("0x7e7025221ea192b6").unwrap());
    }

    #[test]
    fn hard2_kats() {
        let fm = FeistelMimc::new(&FM_PARAMS_HARD2);
        let input: [Scalar; 2] = [Scalar::zero(), Scalar::one()];
        let perm = fm.permutation(&input);
        assert_eq!(perm[0], from_hex("0x095c81195f93fa60").unwrap());
        assert_eq!(perm[1], from_hex("0x41c45da5e1655eb6").unwrap());
    }
}
