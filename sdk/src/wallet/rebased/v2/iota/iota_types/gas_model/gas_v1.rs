// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use checked::*;

//#[iota_macros::with_checked_arithmetic]
mod checked {
    use crate::wallet::rebased::RebasedError;
    use crate::wallet::rebased::v2::iota::iota_protocol_config::ProtocolConfig;
    use crate::wallet::rebased::v2::iota::iota_types::base_types::ObjectID;
    use crate::wallet::rebased::v2::iota::iota_types::gas::{self};
    use crate::wallet::rebased::v2::iota::iota_types::gas_model::gas_predicates::cost_table_for_version;
    use crate::wallet::rebased::v2::iota::iota_types::gas_model::tables::GasStatus;
    use crate::wallet::rebased::v2::iota::iota_types::gas_model::tables::ZERO_COST_SCHEDULE;
    use crate::wallet::rebased::v2::iota::iota_types::gas_model::units_types::CostTable;
    use crate::wallet::rebased::v2::iota::transaction::ObjectReadResult;

    pub type UserInputResult<T = ()> = Result<T, RebasedError>;

    #[derive(Debug)]
    pub struct IotaGasStatus {
        /// GasStatus as used by the VM, that is all the VM sees
        pub gas_status: GasStatus,
        /// Cost table contains a set of constant/config for the gas
        /// model/charging
        cost_table: IotaCostTable,
        /// Gas budget for this gas status instance.
        /// Typically the gas budget as defined in the
        /// `TransactionData::GasData`
        gas_budget: u64,
        /// Computation cost after execution. This is the result of the gas used
        /// by the `GasStatus` properly bucketized.
        /// Starts at 0 and it is assigned in `bucketize_computation`.
        computation_cost: u64,
        /// Whether to charge or go unmetered
        charge: bool,
        /// Gas price for computation.
        /// This is a multiplier on the final charge as related to the RGP
        /// (reference gas price). Checked at signing: `gas_price >=
        /// reference_gas_price` and then conceptually
        /// `final_computation_cost = total_computation_cost * gas_price /
        /// reference_gas_price`
        gas_price: u64,
        // Reference gas price as defined in protocol config.
        // If `protocol_defined_base_fee' is enabled, this is a mandatory base fee paid to the
        // protocol.
        reference_gas_price: u64,
        /// Gas price for storage. This is a multiplier on the final charge
        /// as related to the storage gas price defined in the system
        /// (`ProtocolConfig::storage_gas_price`).
        /// Conceptually, given a constant `obj_data_cost_refundable`
        /// (defined in `ProtocolConfig::obj_data_cost_refundable`)
        /// `total_storage_cost = storage_bytes * obj_data_cost_refundable`
        /// `final_storage_cost = total_storage_cost * storage_gas_price`
        storage_gas_price: u64,
        /// Per Object Storage Cost and Storage Rebate, used to get accumulated
        /// values at the end of execution to determine storage charges
        /// and rebates.
        per_object_storage: Vec<(ObjectID, PerObjectStorage)>,
        // storage rebate rate as defined in the ProtocolConfig
        rebate_rate: u64,
        /// Amount of storage rebate accumulated when we are running in
        /// unmetered mode (i.e. system transaction). This allows us to
        /// track how much storage rebate we need to retain in system
        /// transactions.
        unmetered_storage_rebate: u64,
        /// Rounding value to round up gas charges.
        gas_rounding_step: Option<u64>,
        /// Flag to indicate whether the protocol-defined base fee is enabled,
        /// in which case the reference gas price is burned.
        protocol_defined_base_fee: bool,
    }

    impl IotaGasStatus {
        fn new(
            move_gas_status: GasStatus,
            gas_budget: u64,
            charge: bool,
            gas_price: u64,
            reference_gas_price: u64,
            storage_gas_price: u64,
            rebate_rate: u64,
            gas_rounding_step: Option<u64>,
            cost_table: IotaCostTable,
            protocol_defined_base_fee: bool,
        ) -> IotaGasStatus {
            let gas_rounding_step = gas_rounding_step.map(|val| val.max(1));
            IotaGasStatus {
                gas_status: move_gas_status,
                gas_budget,
                charge,
                computation_cost: 0,
                gas_price,
                reference_gas_price,
                storage_gas_price,
                per_object_storage: Vec::new(),
                rebate_rate,
                unmetered_storage_rebate: 0,
                gas_rounding_step,
                cost_table,
                protocol_defined_base_fee,
            }
        }

        pub fn new_unmetered() -> IotaGasStatus {
            Self::new(
                GasStatus::new_unmetered(),
                0,
                false,
                0,
                0,
                0,
                0,
                None,
                IotaCostTable::unmetered(),
                false,
            )
        }

        pub(crate) fn new_with_budget(
            gas_budget: u64,
            gas_price: u64,
            reference_gas_price: u64,
            config: &ProtocolConfig,
        ) -> IotaGasStatus {
            let storage_gas_price = config.storage_gas_price();
            let max_computation_budget = config.max_gas_computation_bucket() * gas_price;
            let computation_budget = if gas_budget > max_computation_budget {
                max_computation_budget
            } else {
                gas_budget
            };
            let iota_cost_table = IotaCostTable::new(config, gas_price);
            let gas_rounding_step = config.gas_rounding_step_as_option();
            Self::new(
                GasStatus::new(
                    iota_cost_table.execution_cost_table.clone(),
                    computation_budget,
                    gas_price,
                    config.gas_model_version(),
                ),
                gas_budget,
                true,
                gas_price,
                reference_gas_price,
                storage_gas_price,
                config.storage_rebate_rate(),
                gas_rounding_step,
                iota_cost_table,
                config.protocol_defined_base_fee(),
            )
        }

        // Check whether gas arguments are legit:
        // 1. Gas object has an address owner.
        // 2. Gas budget is between min and max budget allowed
        // 3. Gas balance (all gas coins together) is bigger or equal to budget
        pub(crate) fn check_gas_balance(&self, gas_objs: &[&ObjectReadResult], gas_budget: u64) -> UserInputResult<()> {
            // 1. All gas objects have an address owner
            for gas_object in gas_objs {
                // if as_object() returns None, it means the object has been deleted (and
                // therefore must be a shared object).
                if let Some(obj) = gas_object.as_object() {
                    if !obj.is_address_owned() {
                        return Err(RebasedError::GasObjectNotOwnedObject(format!("owner: {}", obj.owner)));
                    }
                } else {
                    // This case should never happen (because gas can't be a shared object), but we
                    // handle this case for future-proofing
                    return Err(RebasedError::MissingGasPayment(format!("")));
                }
            }

            // 2. Gas budget is between min and max budget allowed
            if gas_budget > self.cost_table.max_gas_budget {
                return Err(RebasedError::GasBudgetTooHigh(format!(
                    "gas_budget: {}, max_budget: {}",
                    gas_budget, self.cost_table.max_gas_budget
                )));
            }

            if gas_budget < self.cost_table.min_transaction_cost {
                return Err(RebasedError::GasBudgetTooLow(format!(
                    "gas_budget: {}, min_budget: {}",
                    gas_budget, self.cost_table.min_transaction_cost
                )));
            }

            // 3. Gas balance (all gas coins together) is bigger or equal to budget
            let mut gas_balance = 0u128;
            for gas_obj in gas_objs {
                // expect is safe because we already checked that all gas objects have an
                // address owner
                gas_balance += gas::get_gas_balance(gas_obj.as_object().expect("object must be owned"))? as u128;
            }
            if gas_balance < gas_budget as u128 {
                Err(RebasedError::GasBalanceTooLow(format!(
                    "gas_balance: {}, needed_gas_amount: {}",
                    gas_balance, gas_budget as u128
                )))
            } else {
                Ok(())
            }
        }
    }

    #[derive(Debug)]
    pub struct PerObjectStorage {
        /// storage_cost is the total storage gas to charge. This is computed
        /// at the end of execution while determining storage charges.
        /// It tracks `storage_bytes * obj_data_cost_refundable` as
        /// described in `storage_gas_price`
        /// It has been multiplied by the storage gas price. This is the new
        /// storage rebate.
        pub storage_cost: u64,
        /// storage_rebate is the storage rebate (in IOTA) for in this object.
        /// This is computed at the end of execution while determining storage
        /// charges. The value is in IOTA.
        pub storage_rebate: u64,
        /// The object size post-transaction in bytes
        pub new_size: u64,
    }

    /// A list of constant costs of various operations in IOTA.
    pub struct IotaCostTable {
        /// A flat fee charged for every transaction. This is also the minimum
        /// amount of gas charged for a transaction.
        pub(crate) min_transaction_cost: u64,
        /// Maximum allowable budget for a transaction.
        pub(crate) max_gas_budget: u64,
        /// Computation cost per byte charged for package publish. This cost is
        /// primarily determined by the cost to verify and link a
        /// package. Note that this does not include the cost of writing
        /// the package to the store.
        package_publish_per_byte_cost: u64,
        /// Per byte cost to read objects from the store. This is computation
        /// cost instead of storage cost because it does not change the
        /// amount of data stored on the db.
        object_read_per_byte_cost: u64,
        /// Unit cost of a byte in the storage. This will be used both for
        /// charging for new storage as well as rebating for deleting
        /// storage. That is, we expect users to get full refund on the
        /// object storage when it's deleted.
        storage_per_byte_cost: u64,
        /// Execution cost table to be used.
        pub execution_cost_table: CostTable,
        /// Computation buckets to cost transaction in price groups
        computation_bucket: Vec<ComputationBucket>,
    }

    impl IotaCostTable {
        pub(crate) fn new(c: &ProtocolConfig, gas_price: u64) -> Self {
            // gas_price here is the Reference Gas Price, however we may decide
            // to change it to be the price passed in the transaction
            let min_transaction_cost = c.base_tx_cost_fixed() * gas_price;
            Self {
                min_transaction_cost,
                max_gas_budget: c.max_tx_gas(),
                package_publish_per_byte_cost: c.package_publish_cost_per_byte(),
                object_read_per_byte_cost: c.obj_access_cost_read_per_byte(),
                storage_per_byte_cost: c.obj_data_cost_refundable(),
                execution_cost_table: cost_table_for_version(c.gas_model_version()),
                computation_bucket: computation_bucket(c.max_gas_computation_bucket()),
            }
        }

        pub(crate) fn unmetered() -> Self {
            Self {
                min_transaction_cost: 0,
                max_gas_budget: u64::MAX,
                package_publish_per_byte_cost: 0,
                object_read_per_byte_cost: 0,
                storage_per_byte_cost: 0,
                execution_cost_table: ZERO_COST_SCHEDULE.clone(),
                // should not matter
                computation_bucket: computation_bucket(5_000_000),
            }
        }
    }

    impl std::fmt::Debug for IotaCostTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // TODO: dump the fields.
            write!(f, "IotaCostTable(...)")
        }
    }

    /// A bucket defines a range of units that will be priced the same.
    /// After execution a call to `GasStatus::bucketize` will round the
    /// computation cost to `cost` for the bucket ([`min`, `max`]) the gas
    /// used falls into.
    #[expect(dead_code)]
    pub(crate) struct ComputationBucket {
        min: u64,
        max: u64,
        cost: u64,
    }

    impl ComputationBucket {
        fn new(min: u64, max: u64, cost: u64) -> Self {
            ComputationBucket { min, max, cost }
        }

        fn simple(min: u64, max: u64) -> Self {
            Self::new(min, max, max)
        }
    }

    // define the bucket table for computation charging
    // If versioning defines multiple functions and
    fn computation_bucket(max_bucket_cost: u64) -> Vec<ComputationBucket> {
        assert!(max_bucket_cost >= 5_000_000);
        vec![
            ComputationBucket::simple(0, 1_000),
            ComputationBucket::simple(1_000, 5_000),
            ComputationBucket::simple(5_000, 10_000),
            ComputationBucket::simple(10_000, 20_000),
            ComputationBucket::simple(20_000, 50_000),
            ComputationBucket::simple(50_000, 200_000),
            ComputationBucket::simple(200_000, 1_000_000),
            ComputationBucket::simple(1_000_000, max_bucket_cost),
        ]
    }
}
