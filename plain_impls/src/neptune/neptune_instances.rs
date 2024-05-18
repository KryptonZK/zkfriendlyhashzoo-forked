use lazy_static::lazy_static;
use std::sync::Arc;

use crate::{
    fields::{bls12::FpBLS12, bn256::FpBN256, f64::F64, st::FpST},
    neptune::neptune_params::NeptuneParams,
};

lazy_static! {
    // BN256
    pub static ref NEPTUNE_BN_PARAMS: Arc<NeptuneParams<FpBN256>> = Arc::new(NeptuneParams::new(4, 5, 6, 68));
    // BLS12
    pub static ref NEPTUNE_BLS_PARAMS: Arc<NeptuneParams<FpBLS12>> = Arc::new(NeptuneParams::new(4, 5, 6, 68));
    // ST
    pub static ref NEPTUNE_ST_PARAMS: Arc<NeptuneParams<FpST>> = Arc::new(NeptuneParams::new(4, 3, 6, 96));
    // Goldilocks
    pub static ref NEPTUNE_GOLDILOCKS_8_PARAMS: Arc<NeptuneParams<F64>> = Arc::new(NeptuneParams::new(8, 7, 6, 38));
    pub static ref NEPTUNE_GOLDILOCKS_12_PARAMS: Arc<NeptuneParams<F64>> = Arc::new(NeptuneParams::new(12, 7, 6, 42));
}
