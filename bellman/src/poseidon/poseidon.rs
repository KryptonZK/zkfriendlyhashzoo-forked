use bellman_ce::pairing::ff::PrimeField;
use std::sync::Arc;

use crate::circuits::Permutation;

use super::poseidon_params::PoseidonParams;

#[derive(Clone, Debug)]
pub struct Poseidon<S: PrimeField> {
    pub(crate) params: Arc<PoseidonParams<S>>,
}

impl<S: PrimeField> Poseidon<S> {
    pub fn new(params: &Arc<PoseidonParams<S>>) -> Self {
        Poseidon {
            params: Arc::clone(params),
        }
    }

    pub fn permutation_not_opt(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state[0] = self.sbox_p(&current_state[0]);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        current_state
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input.iter().map(|el| self.sbox_p(el)).collect()
    }

    fn sbox_p(&self, input: &S) -> S {
        let mut input2 = *input;
        input2.square();

        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(input);
                out
            }
            5 => {
                let mut out = input2;
                out.square();
                out.mul_assign(input);
                out
            }
            _ => {
                panic!();
            }
        }
    }

    fn cheap_matmul(&self, input: &[S], r: usize) -> Vec<S> {
        let v = &self.params.v[r];
        let w_hat = &self.params.w_hat[r];
        let t = self.params.t;

        let mut new_state = vec![S::zero(); t];
        new_state[0] = self.params.mds[0][0];
        new_state[0].mul_assign(&input[0]);
        for i in 1..t {
            let mut tmp = w_hat[i - 1];
            tmp.mul_assign(&input[i]);
            new_state[0].add_assign(&tmp);
        }
        for i in 1..t {
            new_state[i] = input[0];
            new_state[i].mul_assign(&v[i - 1]);
            new_state[i].add_assign(&input[i]);
        }

        new_state
    }

    fn matmul(&self, input: &[S], mat: &[Vec<S>]) -> Vec<S> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![S::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate() {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn add_rc(&self, input: &[S], rc: &[S]) -> Vec<S> {
        input
            .iter()
            .zip(rc.iter())
            .map(|(a, b)| {
                let mut r = *a;
                r.add_assign(b);
                r
            })
            .collect()
    }
}

impl<S: PrimeField> Permutation<S> for Poseidon<S> {
    fn permutation(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();
        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        current_state = self.add_rc(&current_state, &self.params.opt_round_constants[0]);
        current_state = self.matmul(&current_state, &self.params.m_i);

        for r in self.params.rounds_f_beginning..p_end {
            current_state[0] = self.sbox_p(&current_state[0]);
            if r < p_end - 1 {
                current_state[0].add_assign(
                    &self.params.opt_round_constants[r + 1 - self.params.rounds_f_beginning][0],
                );
            }
            current_state = self.cheap_matmul(&current_state, p_end - r - 1);
        }
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

#[cfg(test)]
mod poseidon_tests_bls12 {
    use super::*;
    use crate::{
        poseidon::poseidon_instance_bls12::{
            POSEIDON_BLS_12_PARAMS, POSEIDON_BLS_3_PARAMS, POSEIDON_BLS_4_PARAMS,
            POSEIDON_BLS_5_PARAMS, POSEIDON_BLS_8_PARAMS, POSEIDON_BLS_9_PARAMS,
        },
        utils,
    };
    use bellman_ce::pairing::{bls12_381, ff::Field, from_hex};

    type Scalar = bls12_381::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_3_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = poseidon.permutation(&input1);
            let perm2 = poseidon.permutation(&input1);
            let perm3 = poseidon.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats3() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_3_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x200e6982ac00df8fa65cef1fde9f21373fdbbfd98f2df1eb5fa04f3302ab0397").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x2233c9a40d91c1f643b700f836a1ac231c3f3a8d438ad1609355e1b7317a47e5").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x2eae6736db3c086ad29938869dedbf969dd9804a58aa228ec467b7d5a08dc765").unwrap()
        );
    }

    #[test]
    fn kats4() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_4_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x3ee96c25cccd2d4ce9423040834a34e61fccbd98d1820ee60d31098867511dc3").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x199425b0c3d883fee473e656c2fd7fcb1505acae38882cc9f700af90c419f631").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x4715733d30978870e3b199c42e836f60b05654bee15e1b7b2727128477e8671f").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x3dcb25fd97232b62619a0eed46588a42f9860cb096789b5263faf3cc1c0cd5fc").unwrap()
        );
    }

    #[test]
    fn kats5() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_5_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x206e9c93bd36f4cb016d190b7b8a4f52d7992c155abbfff3b855aea261f79aa8").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x34dc8d90ebcde875fbaec4db621211ac2c8d20b9aa811808cc48bac712bfcd6c").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x5fe0b17748603991835b11afb207953781c1894df3390f916c6f07fd90eb272b").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x2975d475ac7018c8395aa0f07ac1dcf62fa93e7f3ecb14db83484708360d49d5").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x3acb636eb50d7533229e8c7a5cd48b866d94763523a452904a807d96e9b67281").unwrap()
        );
    }

    #[test]
    fn kats8() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_8_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
            utils::from_u64::<Scalar>(5),
            utils::from_u64::<Scalar>(6),
            utils::from_u64::<Scalar>(7),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x13f2636c78f8e8d59aa615fa669b68d23dc89d47b26f06052afcbc1c0933ffbe").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x0d953cbd9befc82614fe8e2c1edbb1b50919ef2eb6bfaefbbe737039e3c5f8a2").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x37db098a67bcbc4b94b62106af44b1b3e9b6d4a5d7dc14fd9dcf0885a0f36888").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x4f48e101ecab61473889ec20261383943034e2a328ba1cd60feb44bec58d151c").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x623c26576cf3b33011ea22b722d7baa173b8efb18c26c54f00c4a413d0b5af7a").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x26b7561edf21d399b7e664ee561c9f5daa918d0d7f19bf8e194fcabf349d1f92").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x03b543d81976eef5c8b1cd462eb30a6c916c668e2db4cfa1a4ce248990a88d13").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x2a5606c6f8784f4afa74a81f1e956315c10a0ca1ab3dd8301740f1dda1040ea9").unwrap()
        );
    }

    #[test]
    fn kats9() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_9_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
            utils::from_u64::<Scalar>(5),
            utils::from_u64::<Scalar>(6),
            utils::from_u64::<Scalar>(7),
            utils::from_u64::<Scalar>(8),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x6cf0db2e4810faeafd72beaf9fe223e0dd16125eb270501e381aaf467759570e").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x01835dcba39a394678e0cb1138379f210e5d9296827530712409305f3d138b4d").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x020c85c3a3236b98fd01dc3a196332e370dcc2117cf7cbfbe7a96822f55068ea").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x0453ece0a5f347042bd1701dc8255c31c22744af230d6c1e79db727d2504f0ea").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x6142bd44a6992ac167130a7e0086942e407a84ce9da9b83cd1c2b4ce8ec03566").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x2aa4b7bf725246b22f4c491b017c711780c63fd37f6eb006d6ce64a920713a8c").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x6fd36f35837cfa867179355a097f01736d66bfa38f52dad2f2d5fdf46983e337").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x5aa5eaaeaa2c9cceb21365e539ed777bfd1189c03e3292d03ab38924246f8ece").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x4f7a21722a25a9b89b28f74b5a2937fb3d948d3839bb2e76d395d5b539ee6b98").unwrap()
        );
    }

    #[test]
    fn kats12() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_12_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
            utils::from_u64::<Scalar>(5),
            utils::from_u64::<Scalar>(6),
            utils::from_u64::<Scalar>(7),
            utils::from_u64::<Scalar>(8),
            utils::from_u64::<Scalar>(9),
            utils::from_u64::<Scalar>(10),
            utils::from_u64::<Scalar>(11),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x46905194c5c31bea9bcdc71bc53d3a6cf1c5f967e11ae1262590bbcb04919f26").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x69644cc5ccc23bbe7ec973ca1d89952b26f81c3f5604ba04ed5a94a9e5b64131").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1d6576f854b33f18c3df6869f5cd1fe20fa16865672e4f3740923dd83be51ae2").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x504e0d36739d785e08c586de84264faca1226d7ce243ad6d88ec8216e1fa0f12").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x3ea28ad2200916fef70ff5841d48511cf6b6161e0d7640e5637c5e0e60419e02").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x473b01bd8c243d16f3b19654478b40768db68f6b2544b1563eba49c335f5eda8").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x5d367a9b48f38a06cf64f2a0db748736ff9e7924e96f720ab76124a0839fe029").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x2ae960282742137118c782402fa9e0d79a043de9f0f843e1d1593305555d4612").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x3a142bd35f3ed54c1b60a9427df6676cf3ce3139894c66cb6a245193fbb44abb").unwrap()
        );
        assert_eq!(
            perm[9],
            from_hex("0x0de3dec8532689b0ed1826a88677b9411dccb5ecfb1bf11e564efaf3a700009f").unwrap()
        );
        assert_eq!(
            perm[10],
            from_hex("0x32f057a8b677fc33f4dc4baff91354b96b49f70fb9ee1dce5c8ac4fd302913f2").unwrap()
        );
        assert_eq!(
            perm[11],
            from_hex("0x56c39848efdcb9626ab7568016f0d745c9bd7d3336dc1bbba88ff89e9b3bc891").unwrap()
        );
    }

    #[test]
    fn opt_equals_not_opt() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_3_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = poseidon.permutation(&input);
            let perm2 = poseidon.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}

#[cfg(test)]
mod poseidon_tests_bn256 {
    use super::*;
    use crate::{
        poseidon::poseidon_instance_bn256::{
            POSEIDON_BN_12_PARAMS, POSEIDON_BN_3_PARAMS, POSEIDON_BN_4_PARAMS,
            POSEIDON_BN_5_PARAMS, POSEIDON_BN_8_PARAMS, POSEIDON_BN_9_PARAMS,
        },
        utils,
    };
    use bellman_ce::pairing::{bn256, ff::Field, from_hex};

    type Scalar = bn256::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let poseidon = Poseidon::new(&POSEIDON_BN_3_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = poseidon.permutation(&input1);
            let perm2 = poseidon.permutation(&input1);
            let perm3 = poseidon.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats3() {
        let poseidon = Poseidon::new(&POSEIDON_BN_3_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0c47f98955525514970fab346fb024f3587442216a8a7504fa356d26dd9fbd71").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x04b6f1f4cfaddda2109f462f5417fa7ce67a51212b137361f5835a6f22896921").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1342b9e6c380f4f708c9ef4e033c4cc544e76e83eccff4f3df3afbacfe52f06e").unwrap()
        );
    }

    #[test]
    fn kats4() {
        let poseidon = Poseidon::new(&POSEIDON_BN_4_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2d31b3d35dc3b7debf3677485b7ac5ea0df4257a52aecc8b874703cce9be7771").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x0532caf2657219b1d699cc804c85c85e5610fd4a1f2c0f034d1202c202cb080a").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x27dd5a62eb65ca3b2e279da1cb2d87bceac794271fab64f0093a5cbd90344b02").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x2f549397693d19581eca48a71513da5aab7a62524a52d4e415bf64df0cd8cee5").unwrap()
        );
    }

    #[test]
    fn kats5() {
        let poseidon = Poseidon::new(&POSEIDON_BN_5_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0bacaf52d08263f7e986bdb77336f7a6b691aad2dc5a4b4e9fbc94aacc728721").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x227c086ad54e2a8c068d76344750c08bf7ba7baab1ea2b5261db8cfb688c2855").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x22e17b86e80c8d3b6602a258d39a344f5f574f3decc575f953cdef3b7ba353b6").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x136ea848514bf4b0fd69b33f726fab33b6167b54241150d7cc67408ed84b6a30").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x1fd1f966eecc44cb3225c085890fdee892bc8ce495f062780541ccbb81e08f53").unwrap()
        );
    }

    #[test]
    fn kats8() {
        let poseidon = Poseidon::new(&POSEIDON_BN_8_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
            utils::from_u64::<Scalar>(5),
            utils::from_u64::<Scalar>(6),
            utils::from_u64::<Scalar>(7),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x257c0eb6eeb92b8923ec6ac2410a6494534f7eaa352d3878d6912be79242dacd").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x0a95b37bfae532d4cc20b9b3de278b42b9868ca4053bd0eb0fbbea62fb70f11d").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1ec2c0490f43d1629ed22b507eaed26a88d15b252692e2d07c34adcb8f0980f3").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x09a260330c64b05d8731fe6faad0571323083391592041b78a164c2c23eb79bb").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x1d5423521e4e702d1c349bb1304872da789b4ca38e4e5e5acd1e91cc77d7a1a8").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x2d7d68cc6a965923ea1f514beec390073a54873b1992c766aa54cd75df6a5b0e").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x1d90f954cac8aa80c2f898a86ff792300063be9613c5880cff6d8901263d7be2").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x1e56c6998db673a08b6c08d9f6fada041781315aa2588c24fa37a8e8b90d0079").unwrap()
        );
    }

    #[test]
    fn kats9() {
        let poseidon = Poseidon::new(&POSEIDON_BN_9_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
            utils::from_u64::<Scalar>(5),
            utils::from_u64::<Scalar>(6),
            utils::from_u64::<Scalar>(7),
            utils::from_u64::<Scalar>(8),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0c73e503368259e8bbb1420bad58c21f7795abf89cb18ef24f0f83083cc524bb").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x070115c3224c5e9d988eff2f74d7154caa7f9660c1d73abcde4c47d5042b623e").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x2132b7dcf55a19797d7c8803e567454b85997ef89b0b245b90b7b96362746d3e").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x2aa4c23c17a918dbcb96488a2066805188b944c4b9f9176c5a63a84e790c4580").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x19ff54d34d1298da372247fea2680813bd88e01e1ff292d03ef33286d585cf4a").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x277986c03894a2ec805a998dcb03b6e38b38fa37af263b739df1244993bcd912").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x1a156dad818c72e48d9e580e4e0317c3850c0c5f09ea56aea4b345dea9e55d8f").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x1b3231b5979256237a9747639e5557136b373c74ebcfbcec71272566a4a4aa02").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x256330562e08ddf613a5d7aebf6324723a3ad00d996c72cc908932ccab3465d8").unwrap()
        );
    }

    #[test]
    fn kats12() {
        let poseidon = Poseidon::new(&POSEIDON_BN_12_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
            utils::from_u64::<Scalar>(5),
            utils::from_u64::<Scalar>(6),
            utils::from_u64::<Scalar>(7),
            utils::from_u64::<Scalar>(8),
            utils::from_u64::<Scalar>(9),
            utils::from_u64::<Scalar>(10),
            utils::from_u64::<Scalar>(11),
        ];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2d2da8a229b4dec38c88ab4a133beac879ae09f978d6fb040e7fcaa20e69810d").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x05e928e3d2a5f69e48786451920e11ec916966434c48e0e1b7734817c372a4fd").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x2a8b291e9287ed70bc3cdc82b1ce6d9f5d93a6a414d11350571d32c4eed884a2").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x08eb7a94f3387d7041729c035e770998151e38f960e90a83a3feb3ff9500a188").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x19f3f314c78ad3cc94c30f6e540d6231616b11ba8fa9c5965b270d5379a714a5").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x1035ef3d768e2e09c608319cf34cf3aacef23c0547b5904a7715939597a85dad").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x06ab31421e1372c5beaae408281b7936272c396deda1b3e474823ca39daf4f39").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x1c61aa0a5c84398b7a9c8113de923228a03896bce3e215af3cb3ec8682a1a228").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x2c8a17b0d2e4617a72efba6f139a80ddf759b30d92420abdfa1fbf064dbc9cf0").unwrap()
        );
        assert_eq!(
            perm[9],
            from_hex("0x1185e49d3bf54a8ffe49d73b802337eb4ccba85d2dd607372aa560c300ecfa6d").unwrap()
        );
        assert_eq!(
            perm[10],
            from_hex("0x2a8b131218ecd0176d6b2b0a2242b5705daf983c0d5106efd635495b87418c5f").unwrap()
        );
        assert_eq!(
            perm[11],
            from_hex("0x1fdfd4290662ce5e32ebb00dac55e311602f7a5c0d2e6f7e976239d867ebe5f9").unwrap()
        );
    }

    #[test]
    fn opt_equals_not_opt() {
        let poseidon = Poseidon::new(&POSEIDON_BN_3_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = poseidon.permutation(&input);
            let perm2 = poseidon.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}
