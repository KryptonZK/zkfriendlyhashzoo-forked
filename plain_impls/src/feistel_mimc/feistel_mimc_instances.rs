use lazy_static::lazy_static;
use std::sync::Arc;

use crate::{
    feistel_mimc::feistel_mimc_params::FeistelMimcParams,
    fields::{bls12::FpBLS12, bn256::FpBN256, st::FpST},
};

lazy_static! {
    // BN256
    pub static ref FM_BN_PARAMS: Arc<FeistelMimcParams<FpBN256>> = Arc::new(FeistelMimcParams::new(5));
    // BLS12
    pub static ref FM_BLS_PARAMS: Arc<FeistelMimcParams<FpBLS12>> = Arc::new(FeistelMimcParams::new(5));
    // ST
    pub static ref FM_ST_PARAMS: Arc<FeistelMimcParams<FpST>> = Arc::new(FeistelMimcParams::new(3));
}
