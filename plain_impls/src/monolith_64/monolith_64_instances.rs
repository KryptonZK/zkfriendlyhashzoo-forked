use lazy_static::lazy_static;
use std::sync::Arc;

use crate::{
    fields::const_f64::ConstF64, fields::f64::F64,
    monolith_64::monolith_64_params::Monolith64Params,
};

lazy_static! {
    pub static ref MONOLITH_64_8_PARAMS: Arc<Monolith64Params<F64, 8>> =
        Arc::new(Monolith64Params::new());
    pub static ref MONOLITH_CONST64_8_PARAMS: Arc<Monolith64Params<ConstF64, 8>> =
        Arc::new(Monolith64Params::new());
    pub static ref MONOLITH_64_12_PARAMS: Arc<Monolith64Params<F64, 12>> =
        Arc::new(Monolith64Params::new());
    pub static ref MONOLITH_CONST64_12_PARAMS: Arc<Monolith64Params<ConstF64, 12>> =
        Arc::new(Monolith64Params::new());
}
