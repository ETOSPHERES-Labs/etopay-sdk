//use effects_v1::TransactionEffectsV1;
// use super::effects::TransactionEffectsV1;

mod effects_v1;
mod object_change;
use crate::wallet::rebased::v2::iota::iota_types::gas::GasCostSummary;
use effects_v1::*;
use object_change::*;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

/// The response from processing a transaction or a certified transaction
#[enum_dispatch(TransactionEffectsAPI)]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum TransactionEffects {
    V1(TransactionEffectsV1),
}

impl TransactionEffects {}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum IDOperation {
    None,
    Created,
    Deleted,
}

#[enum_dispatch]
pub trait TransactionEffectsAPI {
    fn gas_cost_summary(&self) -> &GasCostSummary;
}
