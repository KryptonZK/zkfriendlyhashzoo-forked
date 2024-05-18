use bellman_ce::pairing::{
    ff::{PrimeField, SqrtField},
    LegendreSymbol,
};
use std::sync::Arc;

use crate::circuits::Permutation;

use super::grendel_params::GrendelParams;

#[derive(Clone, Debug)]
pub struct Grendel<S: PrimeField + SqrtField> {
    pub(crate) params: Arc<GrendelParams<S>>,
}

impl<S: PrimeField + SqrtField> Grendel<S> {
    pub fn new(params: &Arc<GrendelParams<S>>) -> Self {
        Grendel {
            params: Arc::clone(params),
        }
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input
            .iter()
            .map(|el| {
                // power
                let mut el2 = *el;
                el2.square();
                let mut res = match self.params.d {
                    2 => el2,
                    3 => {
                        let mut out = el2;
                        out.mul_assign(el);
                        out
                    }
                    5 => {
                        let mut out = el2;
                        out.square();
                        out.mul_assign(el);
                        out
                    }
                    _ => {
                        panic!();
                    }
                };
                // legendre
                let symbol = el.legendre();
                match symbol {
                    LegendreSymbol::QuadraticNonResidue => res.negate(),
                    LegendreSymbol::QuadraticResidue => {}
                    _ => res = S::zero(),
                }
                res
            })
            .collect()
    }

    fn affine(&self, input: &[S], round: usize) -> Vec<S> {
        let mat_result = self.matmul(input, &self.params.mds);
        Self::add_rc(&mat_result, &self.params.round_constants[round])
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

    fn add_rc(input: &[S], round_constants: &[S]) -> Vec<S> {
        debug_assert!(input.len() == round_constants.len());
        input
            .iter()
            .zip(round_constants.iter())
            .map(|(a, b)| {
                let mut r = *a;
                r.add_assign(b);
                r
            })
            .collect()
    }
}

impl<S: PrimeField + SqrtField> Permutation<S> for Grendel<S> {
    fn permutation(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state);
            current_state = self.affine(&current_state, r);
        }

        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

#[cfg(test)]
mod grendel_tests_bls12 {
    use super::*;
    use crate::{grendel::grendel_instance_bls12::*, utils};
    use bellman_ce::pairing::{bls12_381, ff::Field, from_hex};

    type Scalar = bls12_381::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let grendel = Grendel::new(&GRENDEL_BLS_3_PARAMS);
        let t = grendel.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = grendel.permutation(&input1);
            let perm2 = grendel.permutation(&input1);
            let perm3 = grendel.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats3() {
        let grendel = Grendel::new(&GRENDEL_BLS_3_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x6bcfcc0e752d6d97f659cda5211d176c6dd78ecdbba20890df80f1b7111aa72d").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x56c9ab01686bdd8701653307d6e5ec6d94de6f4fc6c18d6ce6fda92efcbcd005").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x3e2d666a2c77a98d793cacfb16b8755961308d6974e545b538d2c61ad27c4861").unwrap(),
        );
    }

    #[test]
    fn kats4() {
        let grendel = Grendel::new(&GRENDEL_BLS_4_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
        ];
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x1251e515bcf2e90df739d028a341a772a388f700b92fced1c9f0f44c165980dd").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x51a85e6bc22a3526ed42113a52446c76bfd07430835450a857579d15c2282f32").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x41a5802bb0370908e350fde18a5bedcce7692ba48ae7d3a8c229e8345423842b").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x52624ecb1e78119e4e675b4c0df1c70ded8f7e99d3938cb3fd8b31830aba5227").unwrap(),
        );
    }

    #[test]
    fn kats5() {
        let grendel = Grendel::new(&GRENDEL_BLS_5_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
        ];
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0e6e64b80c9965f375daaceb9e10c4db7cebfd1dc526c0511ca3d2d1c0c1315e").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x2811fcc1f647d1f9db89ce8e6c2e09216dfc67481a200a48f161eec9e2bc81ff").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x40038da75ed81bffcfdcce5c8d83da6355c2e8c36b82e496cde652ff1098eff3").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x5876d0ed269aec7d6ebf49208c64705eabc20d84cd225ab28b95bec4ff22aa13").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x65cf6e2a2a470f5dc8a4a953372561aa6ee31e828cc50bf254bfd3b67ecd2b1f").unwrap(),
        );
    }

    #[test]
    fn kats8() {
        let grendel = Grendel::new(&GRENDEL_BLS_8_PARAMS);
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
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x4ea71d66b61ae1820f58f6c1d849e391d7180a48d674a35de434cf2d467a444e").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x6f0a705ad4140d7c2b25ed8406b4f87ea0d7bb2be09f25dce048321d74464126").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x5fa0a6b920eacee1f386cd3d3036f3b1517ec795e3204e13dee075484bfe95d2").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x389df03e53fc75e03410446caad4eeef584fc086e8c1e64af9d41acba068c4a7").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x05f2e4a67f5e6b9c6c0e8191f24a1b9164e9e1e630dcd2c90716a7074d7fd1c2").unwrap(),
        );
        assert_eq!(
            perm[5],
            from_hex("0x696a0d79aae5db43ee4d76eea1f75c6359d3c67723034722fa277baa70802559").unwrap(),
        );
        assert_eq!(
            perm[6],
            from_hex("0x25e5548ee7740d0967b94bbd2fb8e93e10f395dfb3cb00b5a4ce04394259f9c3").unwrap(),
        );
        assert_eq!(
            perm[7],
            from_hex("0x18ca95141a7295b96b641479e8c9249714bcfa90575d7a3c21f2e4daa4e98a6d").unwrap(),
        );
    }

    #[test]
    fn kats9() {
        let grendel = Grendel::new(&GRENDEL_BLS_9_PARAMS);
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
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x217e80482cd6f2c75ea06245afa144b23ed547be20b8b0f7ea96b8d2e9e4af76").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x197e15d96557f9afaeda20fdc3670363ae8e2c89c96f40deed520efad00df491").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x53f7b1dbc333f84ccdd5a2ecc6f108950d96b034881239ec2fba45a3aefc8bd9").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x27fbeb27f2f881200d3ddc916534689d4f7c25342f278326a10015be2b7e0e1e").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x43a3db23d95f6466614ca780fa1e5caae80d869ef8b103abe87496ba268a0917").unwrap(),
        );
        assert_eq!(
            perm[5],
            from_hex("0x45e7c48562dd03b57099cbbc2f312c6b125c24bd94e550b8e0c0c931fc815c18").unwrap(),
        );
        assert_eq!(
            perm[6],
            from_hex("0x565947d8d377527d2a4e690115b48400c938a8346f48c3d56fa4a9dc0408a100").unwrap(),
        );
        assert_eq!(
            perm[7],
            from_hex("0x006bd7da12667f54d5a2b4111ae903ab2f49ff8110e2a9c0262dc9054a526453").unwrap(),
        );
        assert_eq!(
            perm[8],
            from_hex("0x3e2e9fba0668ec56112d37ee77752b577ed3698b1b9453103d7444361b762eb5").unwrap(),
        );
    }

    #[test]
    fn kats12() {
        let grendel = Grendel::new(&GRENDEL_BLS_12_PARAMS);
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
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x65edfd9fbcca9b6b273de66ee08d50b0c0e097c21571932ff63a1f4d271e352d").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x573e98a91764586915eb3cc175c72a7fe9464b5b13fb07df922bd1fed56b7dd5").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x64a0d97ab0fe5e3e1e66ffae4f4edd0b03f7cd748799c7cee672e23f0e4d9359").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x065b6e8d9431fcd57d3435edcfec5cae3ec0603a762e58c4691beb3bfa954825").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x391cef0ab723c6044283978ef46f13d93825943f1934c9c53d975686d315c9b4").unwrap(),
        );
        assert_eq!(
            perm[5],
            from_hex("0x648fe557df342eeb422f3b961bb57cf6f15e497104eaaa7a1dddb003c7b27000").unwrap(),
        );
        assert_eq!(
            perm[6],
            from_hex("0x03b048dfc1ea7bf910711b0f8306273b9881d0d7f153e3d7f4b0d2ca0dad9fbf").unwrap(),
        );
        assert_eq!(
            perm[7],
            from_hex("0x33c9c1d021c69182135fd00d0a0e4aaf03c3cb8ef8c0b3a619b5e5b162852e76").unwrap(),
        );
        assert_eq!(
            perm[8],
            from_hex("0x319b0d63f76bc7747e6142b6e3aa32a8add1aeb933cb6ca8cccbefa6abaf6837").unwrap(),
        );
        assert_eq!(
            perm[9],
            from_hex("0x0e0eca06ea25c6624e40a7066b208c489aaa3860c4e464d644de855b7d0635fb").unwrap(),
        );
        assert_eq!(
            perm[10],
            from_hex("0x0bdedfdb0f7139efc12073b8e5565a74798a0db3e9f2fc14ba2cf222a6da2578").unwrap(),
        );
        assert_eq!(
            perm[11],
            from_hex("0x0d4ef5004aead171802e695da206e09ecae256b6f98b64bf433e4226a2b2a662").unwrap(),
        );
    }
}

#[cfg(test)]
mod grendel_tests_bn256 {
    use super::*;
    use crate::{grendel::grendel_instance_bn256::*, utils};
    use bellman_ce::pairing::{bn256, ff::Field, from_hex};

    type Scalar = bn256::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let grendel = Grendel::new(&GRENDEL_BN_3_PARAMS);
        let t = grendel.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = grendel.permutation(&input1);
            let perm2 = grendel.permutation(&input1);
            let perm3 = grendel.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats3() {
        let grendel = Grendel::new(&GRENDEL_BN_3_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x29c77fe81f39300e9b1e84c25d258f59bbe396b6770dc13eb86f5df42f81991b").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x1757ac08bac739ae6ab444b94b56dda2ae45658c58f62f6ce533f08067329041").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x15f8ab23597e080f2aded491a0e64e03619f4c2691de14684586f6cbf534ff61").unwrap(),
        );
    }

    #[test]
    fn kats4() {
        let grendel = Grendel::new(&GRENDEL_BN_4_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
        ];
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x026b3c77415cdad4ab43a4086f312011c1ab6d57893c54d297383a18a25ab16f").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x02754eca79866d51cb360abd355434b73c59ea4d60d12f5bcf534b5e26d2a162").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x16463bc376c6e5e840963c6ef13914f883dd02f6c17f68a12609e96dea2aece8").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x2d185c1fb5bfde14825fe7d94702e306c6b1994526a625e43d4128403ba08679").unwrap(),
        );
    }

    #[test]
    fn kats5() {
        let grendel = Grendel::new(&GRENDEL_BN_5_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
        ];
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x28f21638c9ab44694742307e502d85648e7287fc1813882eb4a213492c63a01f").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x0b81f3f39dee0a4c0fc83b2b9602b9bfb708114d758ffa4278d0912629800cf8").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x12899a0549d1b4afcd6c9de1c6f66500c31fcb17a9e94e30e2bda8a2f0aa9349").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x178e1a76f59fea9eb8bcde972b17884517d22e3726f7b4ee1348d13a2493e2fc").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x2ac3254dcaedf4b271ea09dd022e8cb70b12da0ad741b5202b23656d87a1bc3d").unwrap(),
        );
    }

    #[test]
    fn kats8() {
        let grendel = Grendel::new(&GRENDEL_BN_8_PARAMS);
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
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x09260df20a823938dbd9941e2022ef0554aa872c2df1cad7085e2f18fcdb21e5").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x2872da856f76b91c4709fb3f3ca3435134735ecfd6730b6bfd47e8577d46f870").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x138097dfe4189daf8a2a6db9eae0eb3a75b1efee4ba48a1bafbf804893db25f6").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x26654c65fce94f29f6f04f1544d132e8520df20a2eb6c12586276f4345d8aaa1").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x1c6f39f0a886a2f49ed8cdcc2a675da590313d5e3ea08179f96e466fbc4125e6").unwrap(),
        );
        assert_eq!(
            perm[5],
            from_hex("0x27018f617fbb6cb8591ce5fb7e77f454cf18a1326b20cab9c730b3f7a84910e0").unwrap(),
        );
        assert_eq!(
            perm[6],
            from_hex("0x1df5a2d08b4342731bb04312124c53d4fcf8408af758c21ead1bdca9897c9d40").unwrap(),
        );
        assert_eq!(
            perm[7],
            from_hex("0x061749182dc6ad030b79ba74826152f6889dd1df4c1cb0d5f5ca9aee789bd407").unwrap(),
        );
    }

    #[test]
    fn kats9() {
        let grendel = Grendel::new(&GRENDEL_BN_9_PARAMS);
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
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2880520a766a64d61c01c8f5734cd0b1ec448168f6c4d2d1269bd494d2a03793").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x0890f34de2cae0299ab95d9bf4912001093278a7e4b543aa46b68dd2c1432069").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x1c6c002ff05c32721ea320003929256657eeab0d890bc8f4553cd55278f0e00e").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x09f5f8f0d0ecf8ed25037f33c1fbfb295af176d375e28704895e00487b72121f").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x2d53fc0261e5706f0f4de3233edc1d594b3237555b58e49f2158898c80709f03").unwrap(),
        );
        assert_eq!(
            perm[5],
            from_hex("0x2e4ea0a817ff39d9dbf600e02d154615757779f0f5fcef291e24078252c99538").unwrap(),
        );
        assert_eq!(
            perm[6],
            from_hex("0x202db1b78870bdd2d6af76e9b48c30379be82d759b488678b23146981c46bec3").unwrap(),
        );
        assert_eq!(
            perm[7],
            from_hex("0x235c50ce9d5869641ad94ed2664cc02592dd589044811a03a60cd155ebbeac91").unwrap(),
        );
        assert_eq!(
            perm[8],
            from_hex("0x071cc79eff097429cf32b596faba18187899652e4f470b7fb2705baff266fc8f").unwrap(),
        );
    }

    #[test]
    fn kats12() {
        let grendel = Grendel::new(&GRENDEL_BN_12_PARAMS);
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
        let perm = grendel.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0a7898b5ac64b06f687b0ed98b51c7394ac386b077d663a2f844086760d53b68").unwrap(),
        );
        assert_eq!(
            perm[1],
            from_hex("0x2d013be6627a9ab95bfed00c1ad3b258143a006663546bc97c2607a7bbb5ab1c").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x0fd57279e8e8f308b6c963aa96afd46bfda5f312178c08698dc0002417feeae7").unwrap(),
        );
        assert_eq!(
            perm[3],
            from_hex("0x2bbd4e1ed3c436ff299fe20024947f236fa16952ba63f7fce70d4e623b00bdbd").unwrap(),
        );
        assert_eq!(
            perm[4],
            from_hex("0x22eacb668f055dfb62e705d1182e227358f905b16784278c11ff63dccfbcfcd1").unwrap(),
        );
        assert_eq!(
            perm[5],
            from_hex("0x1cde969cf23392fcad3c175785639b064584b16cc522f3f2d0fdece80b38f234").unwrap(),
        );
        assert_eq!(
            perm[6],
            from_hex("0x019f879d4b83cf1714eb6aea97d158672712d3c1e60d0562dd92279a3a7a3c0e").unwrap(),
        );
        assert_eq!(
            perm[7],
            from_hex("0x0348349f1f2f6fdaf2a38ee8e66e6e9f3f8238ac762b907738bf5f08e6bf1e2f").unwrap(),
        );
        assert_eq!(
            perm[8],
            from_hex("0x18f176996f1e60f08a8cfe200fbb788387d55a981de0743c43fcc7916093792a").unwrap(),
        );
        assert_eq!(
            perm[9],
            from_hex("0x0d465b04c998b7246a4f7e52f03d3789f91f961d5afc2c0a8def4fc5b0aebcf6").unwrap(),
        );
        assert_eq!(
            perm[10],
            from_hex("0x173a17ac597a26dd2a4e5b4ec5f65a5873b166998b53e97229bbee8379b66a23").unwrap(),
        );
        assert_eq!(
            perm[11],
            from_hex("0x2d721a423ac09a5b77bbad7946b16c8fd02b2a7ee78a7340472eeddc6601ed64").unwrap(),
        );
    }
}
