use super::neptune_params::NeptuneParams;
use bellman_ce::pairing::{bls12_381, bn256};

use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    // BLS12_381
    pub static ref NEPTUNE_BLS_4_PARAMS: Arc<NeptuneParams<bls12_381::Fr>> = Arc::new(NeptuneParams::new(4, 5, 6, 68));
    pub static ref NEPTUNE_BLS_8_PARAMS: Arc<NeptuneParams<bls12_381::Fr>> = Arc::new(NeptuneParams::new(8, 5, 6, 72));
    pub static ref NEPTUNE_BLS_12_PARAMS: Arc<NeptuneParams<bls12_381::Fr>> = Arc::new(NeptuneParams::new(12, 5, 6, 78));
    pub static ref NEPTUNE_BLS_16_PARAMS: Arc<NeptuneParams<bls12_381::Fr>> = Arc::new(NeptuneParams::new(16, 5, 6, 83));
    pub static ref NEPTUNE_BLS_20_PARAMS: Arc<NeptuneParams<bls12_381::Fr>> = Arc::new(NeptuneParams::new(20, 5, 6, 87));
    pub static ref NEPTUNE_BLS_24_PARAMS: Arc<NeptuneParams<bls12_381::Fr>> = Arc::new(NeptuneParams::new(24, 5, 6, 92));
    // BN256
    pub static ref NEPTUNE_BN_4_PARAMS: Arc<NeptuneParams<bn256::Fr>> = Arc::new(NeptuneParams::new(4, 5, 6, 68));
    pub static ref NEPTUNE_BN_8_PARAMS: Arc<NeptuneParams<bn256::Fr>> = Arc::new(NeptuneParams::new(8, 5, 6, 72));
    pub static ref NEPTUNE_BN_12_PARAMS: Arc<NeptuneParams<bn256::Fr>> = Arc::new(NeptuneParams::new(12, 5, 6, 78));
    pub static ref NEPTUNE_BN_16_PARAMS: Arc<NeptuneParams<bn256::Fr>> = Arc::new(NeptuneParams::new(16, 5, 6, 83));
    pub static ref NEPTUNE_BN_20_PARAMS: Arc<NeptuneParams<bn256::Fr>> = Arc::new(NeptuneParams::new(20, 5, 6, 87));
    pub static ref NEPTUNE_BN_24_PARAMS: Arc<NeptuneParams<bn256::Fr>> = Arc::new(NeptuneParams::new(24, 5, 6, 92));
}
