use std::collections::BTreeMap;
use std::sync::LazyLock;

use crate::wallet::rebased::v2::iota::gas_model::units_types::{CostTable, Gas};
use crate::wallet::rebased::v2::mowe::move_core_types::gas_algebra::InternalGas;
use crate::wallet::rebased::v2::mowe::move_vm_profiler::GasProfiler;

pub fn zero_cost_schedule() -> CostTable {
    let mut zero_tier = BTreeMap::new();
    zero_tier.insert(0, 0);
    CostTable {
        instruction_tiers: zero_tier.clone(),
        stack_size_tiers: zero_tier.clone(),
        stack_height_tiers: zero_tier,
    }
}

pub static ZERO_COST_SCHEDULE: LazyLock<CostTable> = LazyLock::new(zero_cost_schedule);

/// The Move VM implementation of state for gas metering.
///
/// Initialize with a `CostTable` and the gas provided to the transaction.
/// Provide all the proper guarantees about gas metering in the Move VM.
///
/// Every client must use an instance of this type to interact with the Move VM.
#[derive(Debug)]
pub struct GasStatus {
    pub gas_model_version: u64,
    cost_table: CostTable,
    gas_left: InternalGas,
    gas_price: u64,
    initial_budget: InternalGas,
    charge: bool,

    // The current height of the operand stack, and the maximal height that it has reached.
    stack_height_high_water_mark: u64,
    stack_height_current: u64,
    stack_height_next_tier_start: Option<u64>,
    stack_height_current_tier_mult: u64,

    // The current (abstract) size  of the operand stack and the maximal size that it has reached.
    stack_size_high_water_mark: u64,
    stack_size_current: u64,
    stack_size_next_tier_start: Option<u64>,
    stack_size_current_tier_mult: u64,

    // The total number of bytecode instructions that have been executed in the transaction.
    instructions_executed: u64,
    instructions_next_tier_start: Option<u64>,
    instructions_current_tier_mult: u64,

    profiler: Option<GasProfiler>,
    num_native_calls: u64,
}

impl GasStatus {
    /// Initialize the gas state with metering enabled.
    ///
    /// Charge for every operation and fail when there is no more gas to pay for
    /// operations. This is the instantiation that must be used when
    /// executing a user script.
    pub fn new(cost_table: CostTable, budget: u64, gas_price: u64, gas_model_version: u64) -> Self {
        assert!(gas_price > 0, "gas price cannot be 0");
        let budget_in_unit = budget / gas_price;
        let gas_left = Self::to_internal_units(budget_in_unit);
        let (stack_height_current_tier_mult, stack_height_next_tier_start) = cost_table.stack_height_tier(0);
        let (stack_size_current_tier_mult, stack_size_next_tier_start) = cost_table.stack_size_tier(0);
        let (instructions_current_tier_mult, instructions_next_tier_start) = cost_table.instruction_tier(0);
        Self {
            gas_model_version,
            gas_left,
            gas_price,
            initial_budget: gas_left,
            cost_table,
            charge: true,
            stack_height_high_water_mark: 0,
            stack_height_current: 0,
            stack_size_high_water_mark: 0,
            stack_size_current: 0,
            instructions_executed: 0,
            stack_height_current_tier_mult,
            stack_size_current_tier_mult,
            instructions_current_tier_mult,
            stack_height_next_tier_start,
            stack_size_next_tier_start,
            instructions_next_tier_start,
            profiler: None,
            num_native_calls: 0,
        }
    }

    /// Initialize the gas state with metering disabled.
    ///
    /// It should be used by clients in very specific cases and when executing
    /// system code that does not have to charge the user.
    pub fn new_unmetered() -> Self {
        Self {
            gas_model_version: 1,
            gas_left: InternalGas::new(0),
            gas_price: 1,
            initial_budget: InternalGas::new(0),
            cost_table: ZERO_COST_SCHEDULE.clone(),
            charge: false,
            stack_height_high_water_mark: 0,
            stack_height_current: 0,
            stack_size_high_water_mark: 0,
            stack_size_current: 0,
            instructions_executed: 0,
            stack_height_current_tier_mult: 0,
            stack_size_current_tier_mult: 0,
            instructions_current_tier_mult: 0,
            stack_height_next_tier_start: None,
            stack_size_next_tier_start: None,
            instructions_next_tier_start: None,
            profiler: None,
            num_native_calls: 0,
        }
    }

    const INTERNAL_UNIT_MULTIPLIER: u64 = 1000;

    fn to_internal_units(val: u64) -> InternalGas {
        InternalGas::new(val * Self::INTERNAL_UNIT_MULTIPLIER)
    }

    // The amount of gas used, it does not include the multiplication for the gas
    // price
    pub fn gas_used_pre_gas_price(&self) -> u64 {
        let gas: Gas = match self.initial_budget.checked_sub(self.gas_left) {
            Some(val) => InternalGas::to_unit_round_down(val),
            None => InternalGas::to_unit_round_down(self.initial_budget),
        };
        u64::from(gas)
    }
}

pub fn initial_cost_schedule_v1() -> CostTable {
    let instruction_tiers: BTreeMap<u64, u64> = vec![
        (0, 1),
        (20_000, 2),
        (50_000, 10),
        (100_000, 50),
        (200_000, 100),
        (10_000_000, 1000),
    ]
    .into_iter()
    .collect();

    let stack_height_tiers: BTreeMap<u64, u64> = vec![(0, 1), (1_000, 2), (10_000, 10)].into_iter().collect();

    let stack_size_tiers: BTreeMap<u64, u64> = vec![
        (0, 1),
        (100_000, 2),        // ~100K
        (500_000, 5),        // ~500K
        (1_000_000, 100),    // ~1M
        (100_000_000, 1000), // ~100M
    ]
    .into_iter()
    .collect();

    CostTable {
        instruction_tiers,
        stack_size_tiers,
        stack_height_tiers,
    }
}
