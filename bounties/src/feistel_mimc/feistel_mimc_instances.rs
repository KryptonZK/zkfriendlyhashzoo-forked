use lazy_static::lazy_static;
use std::sync::Arc;

use crate::{feistel_mimc::feistel_mimc_params::FeistelMimcParams, fields::field64::Fp64};

type Scalar = Fp64;

lazy_static! {
    pub static ref FM_PARAMS_EASY1: Arc<FeistelMimcParams<Scalar>> =
        Arc::new(FeistelMimcParams::new(3, 22));
    pub static ref FM_PARAMS_EASY2: Arc<FeistelMimcParams<Scalar>> =
        Arc::new(FeistelMimcParams::new(3, 25));
    pub static ref FM_PARAMS_MEDIUM: Arc<FeistelMimcParams<Scalar>> =
        Arc::new(FeistelMimcParams::new(3, 30));
    pub static ref FM_PARAMS_HARD1: Arc<FeistelMimcParams<Scalar>> =
        Arc::new(FeistelMimcParams::new(3, 35));
    pub static ref FM_PARAMS_HARD2: Arc<FeistelMimcParams<Scalar>> =
        Arc::new(FeistelMimcParams::new(3, 40));
}
