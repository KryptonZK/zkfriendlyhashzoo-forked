use super::griffin_params::GriffinParams;
use bellman_ce::pairing::{bls12_381, bn256};

use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    // BLS12_381
    pub static ref GRIFFIN_BLS_3_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(3, 5, 12));
    pub static ref GRIFFIN_BLS_4_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(4, 5, 11));
    pub static ref GRIFFIN_BLS_8_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(8, 5, 9));
    pub static ref GRIFFIN_BLS_12_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(12, 5, 9));
    pub static ref GRIFFIN_BLS_16_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(16, 5, 9));
    pub static ref GRIFFIN_BLS_20_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(20, 5, 9));
    pub static ref GRIFFIN_BLS_24_PARAMS: Arc<GriffinParams<bls12_381::Fr>> =
        Arc::new(GriffinParams::new(24, 5, 9));
    // BN256
    pub static ref GRIFFIN_BN_3_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(3, 5, 12));
    pub static ref GRIFFIN_BN_4_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(4, 5, 11));
    pub static ref GRIFFIN_BN_8_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(8, 5, 9));
    pub static ref GRIFFIN_BN_12_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(12, 5, 9));
    pub static ref GRIFFIN_BN_16_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(16, 5, 9));
    pub static ref GRIFFIN_BN_20_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(20, 5, 9));
    pub static ref GRIFFIN_BN_24_PARAMS: Arc<GriffinParams<bn256::Fr>> =
        Arc::new(GriffinParams::new(24, 5, 9));
}
