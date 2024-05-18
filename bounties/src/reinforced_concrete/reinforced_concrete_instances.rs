use lazy_static::lazy_static;
use std::sync::Arc;

use crate::{
    fields::{field48::Fp48, field56::Fp56, field64::Fp64},
    reinforced_concrete::reinforced_concrete_params::ReinforcedConcreteParams,
};

lazy_static! {
    pub static ref SI_EASY: Vec<u16> = vec![267, 267, 267, 244, 258, 235];
    pub static ref SI_MEDIUM: Vec<u16> = vec![638, 659, 635, 646, 659, 634];
    pub static ref SI_HARD: Vec<u16> = vec![570, 577, 549, 579, 553, 577, 553];
    pub static ref RC_PARAMS_EASY: Arc<ReinforcedConcreteParams<Fp48>> = Arc::new(
        ReinforcedConcreteParams::new(3, &SI_EASY, 223, &[1, 2, 3, 4])
    );
    pub static ref RC_PARAMS_MEDIUM: Arc<ReinforcedConcreteParams<Fp56>> = Arc::new(
        ReinforcedConcreteParams::new(3, &SI_MEDIUM, 617, &[1, 3, 2, 4])
    );
    pub static ref RC_PARAMS_HARD: Arc<ReinforcedConcreteParams<Fp64>> = Arc::new(
        ReinforcedConcreteParams::new(3, &SI_HARD, 541, &[1, 3, 2, 4])
    );
}
