use lazy_static::lazy_static;
use std::sync::Arc;

use crate::{
    fields::const_f31::ConstF31, fields::f31::F31,
    monolith_31::monolith_31_params::Monolith31Params,
};

lazy_static! {
    pub static ref MONOLITH_31_16_PARAMS: Arc<Monolith31Params<F31, 16>> =
        Arc::new(Monolith31Params::new());
    pub static ref MONOLITH_CONST31_16_PARAMS: Arc<Monolith31Params<ConstF31, 16>> =
        Arc::new(Monolith31Params::new());
    pub static ref MONOLITH_31_24_PARAMS: Arc<Monolith31Params<F31, 24>> =
        Arc::new(Monolith31Params::new());
    pub static ref MONOLITH_CONST31_24_PARAMS: Arc<Monolith31Params<ConstF31, 24>> =
        Arc::new(Monolith31Params::new());
}
