use bellman_ce::pairing::ff::PrimeField;
use std::sync::Arc;

use crate::circuits::Permutation;

use super::rescue_params::RescueParams;

#[derive(Clone, Debug)]
pub struct Rescue<S: PrimeField> {
    pub(crate) params: Arc<RescueParams<S>>,
}

impl<S: PrimeField> Rescue<S> {
    pub fn new(params: &Arc<RescueParams<S>>) -> Self {
        Rescue {
            params: Arc::clone(params),
        }
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input
            .iter()
            .map(|el| {
                let mut el2 = *el;
                el2.square();

                match self.params.d {
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
                }
            })
            .collect()
    }

    fn sbox_inverse(&self, input: &[S]) -> Vec<S> {
        input.iter().map(|el| el.pow(self.params.d_inv)).collect()
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

impl<S: PrimeField> Permutation<S> for Rescue<S> {
    fn permutation(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state);
            current_state = self.affine(&current_state, 2 * r);
            current_state = self.sbox_inverse(&current_state);
            current_state = self.affine(&current_state, 2 * r + 1);
        }

        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

#[cfg(test)]
mod rescue_tests_bls12 {
    use super::*;
    use crate::{
        rescue::rescue_instance_bls12::{
            RESCUE_BLS_12_PARAMS, RESCUE_BLS_3_PARAMS, RESCUE_BLS_4_PARAMS, RESCUE_BLS_5_PARAMS,
            RESCUE_BLS_8_PARAMS, RESCUE_BLS_9_PARAMS,
        },
        utils,
    };
    use bellman_ce::pairing::{bls12_381, ff::Field, from_hex};

    type Scalar = bls12_381::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue = Rescue::new(&RESCUE_BLS_3_PARAMS);
        let t = rescue.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue.permutation(&input1);
            let perm2 = rescue.permutation(&input1);
            let perm3 = rescue.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats3() {
        let rescue = Rescue::new(&RESCUE_BLS_3_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2e1183b4ae571061ed9514118392ede2904ae1376d61653de09083cf0b31abce").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x38f9e521c67c329a53403dd42999b19c3bfe355e594752c87ada74da35c74b85").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x69a193e3c2734c26d85d191a1e521c1bc8024c9047bb5c79835ed5cfc2d8440e").unwrap(),
        );
    }

    #[test]
    fn kats4() {
        let rescue = Rescue::new(&RESCUE_BLS_4_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
        ];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x1169ad481dc518ec633b4cf403ea15c832a9fe35697519593f9839347970fe4d").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x421c5f9a03306ca97c6e9903a7f8e4b6c8d3e2a74ffb40ba9ae2c99ef4a0d4d1").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x31c1f889aabb54dcc9a9400b03893c31a4b224a34e89d62e39f6d96adf22b227").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x3cefea8ea6457795e2d17f64546c0fed604f7ee331463be69bb714542caae3b7").unwrap()
        );
    }

    #[test]
    fn kats5() {
        let rescue = Rescue::new(&RESCUE_BLS_5_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
        ];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x203fe59f59e040503c777a937fed5c8b0c12151f812a24489bc791531a2ff812").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x53e0007b5e4cd86f0b4c9d1ee0ee5ac62604ee4ea10974f35d288a10daf92563").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x4b551a3b956f548f0e388c38bd7a9e0bf680ac402bbbd357175f98c1d8795aac").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x6c283eb48f1f92606e849a249187dd41d7a641b9a0f3992bf784e8eb5ce9c6ae").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x5baf1851a2da46215edb4576c41510c928994bc23415f5168eeae9b5b5d9ddab").unwrap()
        );
    }

    #[test]
    fn kats8() {
        let rescue = Rescue::new(&RESCUE_BLS_8_PARAMS);
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
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x440f8c7acde465fbab3b9ae8ab869348fc6ce3733c1084ddd399f2827af7b42c").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x6730c746fe4ed98525a652426aa8ae31eb234d07236cd585f6121c62f74bf852").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x070b867bf4db973eb889724e022d264e2afa9b60028186748a39bde06371d941").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x2a5bade7b17dc907b5840c976d19d39f7ec17446c2a3fc7396447e8d236be23c").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x6f667bbcd034ad768624a8dc135b94d3103ea194787ecb5abd017f16b627d8b3").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x12e80327cd3445b70ad3c593f2edb1ca680b59cb874adf79babcd69e697ea487").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x04b1a1015040ee67c445318bdafe70d7eddb6d6e81d6fca9432dd3cb15d9a792").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x2fd5164d5d4c2ffd3596f7bc0ce6a92e8cd5424d1188d7d32a7bdf4c3bc4faa9").unwrap()
        );
    }

    #[test]
    fn kats9() {
        let rescue = Rescue::new(&RESCUE_BLS_9_PARAMS);
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
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x5a93d31be400b8eab327b255764ca8c1e79af0bb02174062470edcce530bcbc5").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x69b47fab0480d89e00102ba8b6c7e7db8336b2f2b9c35c824372033d70ef7964").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x66eddaee278852064844df7fa38094ad42ac28d973b7d12cfddfee05bf64674d").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x53276b569a2efd1f8e1b8e5ac801db8b9431fe68b2070b35d3931c0e1ca5765a").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x25c46287c25f50ab77e5cce0eaf5c3253efabf7531fcfd9611dabec74cf03560").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x1b372529ca78180c1ee6e3e17492c569f32910930080d26bd778d0b961358f89").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x1167fdac5eb41cc9adb9f4154374e7a3ddb1ba83c750ea4369a31a2142a35e70").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x6f6a4802f81a79db50b35b4224a242c2270c6782c4a2550765d8a0ff79040d58").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x0081d74365b4d3f4bd6285b0181cd5993a2acb4d665aaa7898cd079864626eb1").unwrap()
        );
    }

    #[test]
    fn kats12() {
        let rescue = Rescue::new(&RESCUE_BLS_12_PARAMS);
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
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x61fec2fdfef91a4c7649db60ab081aac038a4cc7b48ff4efacce4c3b6a3de79d").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x53e6f46ab00303d2ee101e066be1525d958b150569fce675d066f5d1b236464d").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x58642d43c7f06a6df904884f8e070f1803a85e90ac2085ce70c26f07f374cd19").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x3d39f6bb8d089344520edfab9b26849a8be1133ff927bf6ea109deb453655a67").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x624da66de7485244c0d2c87f8dd866c7a061c26fb9b3b78097312bb2c31a1eaf").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x1b2615f18b7507eb4c74499f9d35def883bb539194cf1d9dce8f91ff14a0d0fc").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x29195fb3f06c669a5f0afe47abab3c96e54d8a89ada8c1b56e2bae7e71b53e3d").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x6619fefeef1e7d299896686b76e8635fe46dbaadfbe4aaceed0539ed003ef156").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x7379d37e235d93821c5912179832d45626f1c6763bbcfda0ef8919c2ecee4802").unwrap()
        );
        assert_eq!(
            perm[9],
            from_hex("0x565c4e54f50b0a1a3d36b9fce41442451abeb8a7f7c415ade77665fdb2cccb46").unwrap()
        );
        assert_eq!(
            perm[10],
            from_hex("0x1c6724a2626b5df1c3648f45f109aa2232cf2e23985b3aca845f7eada36e1f95").unwrap()
        );
        assert_eq!(
            perm[11],
            from_hex("0x66b8b0cc9d49dfa8052efaf7ed03cac0fe6a7cac5ec6115fa4473d4bbe3f5210").unwrap()
        );
    }
}

#[cfg(test)]
mod rescue_tests_bn256 {
    use super::*;
    use crate::{
        rescue::rescue_instance_bn256::{
            RESCUE_BN_12_PARAMS, RESCUE_BN_3_PARAMS, RESCUE_BN_4_PARAMS, RESCUE_BN_5_PARAMS,
            RESCUE_BN_8_PARAMS, RESCUE_BN_9_PARAMS,
        },
        utils,
    };
    use bellman_ce::pairing::{bn256, ff::Field, from_hex};

    type Scalar = bn256::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue = Rescue::new(&RESCUE_BN_3_PARAMS);
        let t = rescue.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue.permutation(&input1);
            let perm2 = rescue.permutation(&input1);
            let perm3 = rescue.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats3() {
        let rescue = Rescue::new(&RESCUE_BN_3_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0dc30ccd5d64e5bea071e99087ef86d433eb156aa0500a823298f9bb05328bd2").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x189893368d5815608c56e44cc67f7e821e093bb6254a0553f9ff69f4d99debc8").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x1acafc768221448ebc51fa2cd1e3c9b2044a0c04f3509d833b0a82c7e3462610").unwrap(),
        );
    }

    #[test]
    fn kats4() {
        let rescue = Rescue::new(&RESCUE_BN_4_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
        ];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x26523b4bf1dbacbd0e792efba63a8984d2bfca05766e70c005d14d736c684308").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x11b3a578e3034f3bf0675137d307ff2749bdf4e5febde7d64e449ff74342c6f7").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x14db80a6c65e0369a731d89320c12a8765053b061598b3536cbf0bd5201b49a9").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x10da0536b84b150a68dfd9468230d5af7577d6c83b173c40f9fe3236d49d934c").unwrap()
        );
    }

    #[test]
    fn kats5() {
        let rescue = Rescue::new(&RESCUE_BN_5_PARAMS);
        let input: Vec<Scalar> = vec![
            Scalar::zero(),
            Scalar::one(),
            utils::from_u64::<Scalar>(2),
            utils::from_u64::<Scalar>(3),
            utils::from_u64::<Scalar>(4),
        ];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2cd8fc0509157f1faf9f730aeb40f6eb745872e281675e39a54b12f60b504dae").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x2888c57a46a166001bd4d59c6ee0996a952fd7321fd1f79dacb31b4729977743").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1fd755076447d23085a92725eb0e9009678be249c091feb5a534fef9a6424402").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x2ee747db42419d1dff124db129c597a15857404c519a595ade6ab8c83fe3fa8f").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x0c3600f7a922d59314a27b26130c152b346c1f4372e9deef67acd6be36b5cd3d").unwrap()
        );
    }

    #[test]
    fn kats8() {
        let rescue = Rescue::new(&RESCUE_BN_8_PARAMS);
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
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0159ce3b869146f10ae5907c84d026ce95ca9b8d55fb00d9df732af73a41cc26").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x27f41998fdae1a972d0dcc498d05449ae45382609c0a793a72e2776a654cce34").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x238e3e6a200abadd986086baa58845f8fa7bdeab53b350b6ce3823ac643a2163").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x0065a6a8d3d86aca8b5a5c488fbced3e1f65d2163c69a42dcda8fad51729452d").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x119c63b47dfc5084a2e23f315b2480832891590efd66306372d3bc8026a94e63").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x12752b017887b563f8704cdaae33430b5924f316c1c8b0d3e9a1591a1add9855").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x1753831d08eea62f9094eb00116b0635c60de73a6511ce1c9daa5d98c7a3c9ee").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x0047c13bb4f0dfc34414b4659196768d65552ae205a622f550e2b169d7e4afd2").unwrap()
        );
    }

    #[test]
    fn kats9() {
        let rescue = Rescue::new(&RESCUE_BN_9_PARAMS);
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
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2ba2a3172863ef17a40468b6a1a2ea1bfee4fa4a761a2907074568a8e99e65ba").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x133f417f83256f9f00c1f3785982af75b7c138e97deb1fc8f5816631a2ebf229").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1d6d6235350a01bc4cddb1fcb93b50d660b10a600b6a9734301f83f5d9148260").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x2bb77e76624ab46be90c7c3d2cc14c412a13d1cdcd5bdba36a0b16ed7962338b").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x1307f5e41af6af33fdc2fb11d41337e012250818e1de80ba359cc8c697dc3740").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x21fe690e86248b2c2be87d5d2cebc65688eee14d17e1c0fe30b268e9e4b4b9d6").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x2b58864293d985c1eb26d2d36820220c95a6c458fc3ecf0a1c604047dc08d854").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x10864e08c67f431951efa913c86b0fd4a32274f34e0c15250b62fe50d4a52fa3").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x0fbebe2b29bd04624634122066dbe2f975332721e9192e6a0e9590613c3669c2").unwrap()
        );
    }

    #[test]
    fn kats12() {
        let rescue = Rescue::new(&RESCUE_BN_12_PARAMS);
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
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x019e0c5da6e63c8d09846fd13284e1e194e5ca14c2b68fd590a6f5d5f6735e60").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x2c8669b5cebc2b854bcd4286c0b3c7404bcb01522364f9691a1d4b5185aaa935").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x2aa904f1ce630259203490a5630dd4a8c33f8105aef646e936bbb3f1043d61ba").unwrap()
        );
        assert_eq!(
            perm[3],
            from_hex("0x005b05d75c053a593bed437f54941db0e81436fac897ced1d77242e8bdc1df15").unwrap()
        );
        assert_eq!(
            perm[4],
            from_hex("0x15997862b69818042e59df7e21d5c75afe54f9b803e44f6bdf9045d70a92f3ab").unwrap()
        );
        assert_eq!(
            perm[5],
            from_hex("0x1367f0cc51ff7745c3380e1ad90acbe64ff89ef036e3dacd9dde6b08c82b9514").unwrap()
        );
        assert_eq!(
            perm[6],
            from_hex("0x0f50445c9efd4b4529b116ce99f5ae795c76700c3cca1da4f727073439b4a118").unwrap()
        );
        assert_eq!(
            perm[7],
            from_hex("0x21c2d7d0103ee8e2263fa076e03658c017bf775cee8c78d8e764debe3e9c32cf").unwrap()
        );
        assert_eq!(
            perm[8],
            from_hex("0x0a074df514042a01e9c0f4fe71a032b01ee1435686bd86723b8173d4f81321a7").unwrap()
        );
        assert_eq!(
            perm[9],
            from_hex("0x0d51b1a6a8cd198058a3edc7894c80360747ad75736b03c860abd8d160f75761").unwrap()
        );
        assert_eq!(
            perm[10],
            from_hex("0x2dcfa8fed7a9a69f26c3e39079a4537a685f8b3f7aa3f84f85b1399c60306308").unwrap()
        );
        assert_eq!(
            perm[11],
            from_hex("0x0d5a23dcb21dec2a89fc72b7925bb45389b24ba5f0debae5e0a4c793c769eab6").unwrap()
        );
    }
}
