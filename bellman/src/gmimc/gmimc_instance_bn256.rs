use super::gmimc_params::GmimcParams;
use bellman_ce::pairing::bn256;

use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = bn256::Fr;
lazy_static! {
    pub static ref GMIMC_BN_3_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(3, 5, 226));
    pub static ref GMIMC_BN_4_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(4, 5, 228));
    pub static ref GMIMC_BN_5_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(5, 5, 230));
    pub static ref GMIMC_BN_8_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(8, 5, 236));
    pub static ref GMIMC_BN_9_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(9, 5, 238));
    pub static ref GMIMC_BN_12_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(12, 5, 314));
    pub static ref GMIMC_BN_16_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(16, 5, 546));
    pub static ref GMIMC_BN_20_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(20, 5, 842));
    pub static ref GMIMC_BN_24_PARAMS: Arc<GmimcParams<Scalar>> =
        Arc::new(GmimcParams::new(24, 5, 1202));
}
