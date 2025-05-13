use crate::wallet::rebased::v2::iota::gas_model::{tables::initial_cost_schedule_v1, units_types::CostTable};

// Return the version supported cost table
pub fn cost_table_for_version(_gas_model: u64) -> CostTable {
    initial_cost_schedule_v1()
}
