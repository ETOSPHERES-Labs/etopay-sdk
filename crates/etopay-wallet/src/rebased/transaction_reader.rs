use chrono::{TimeZone, Utc};

use crate::types::WalletTxStatus;

use super::{
    IotaAddress, IotaCallArg, IotaPureDecodedValue, IotaTransactionBlockEffects, IotaTransactionBlockResponse,
};

pub struct TransactionReader<'a> {
    tx: &'a IotaTransactionBlockResponse,
}

impl<'a> TransactionReader<'a> {
    pub fn new(tx: &'a IotaTransactionBlockResponse) -> Self {
        Self { tx }
    }

    pub fn amount(&self) -> u64 {
        let Some(transaction) = self.tx.transaction.clone() else {
            return 0;
        };

        #[allow(unreachable_patterns)]
        match transaction.data {
            super::IotaTransactionBlockData::V1(iota_transaction_block_data_v1) => {
                let t = iota_transaction_block_data_v1.transaction;

                match t {
                    super::IotaTransactionBlockKind::ProgrammableTransaction(programmable_transaction) => {
                        let amount: u64 = programmable_transaction.inputs.iter().filter_map(extract_u64).sum();
                        amount
                    }
                }
            }
            _ => 0,
        }
    }

    pub fn sender(&self) -> IotaAddress {
        let Some(transaction) = self.tx.transaction.clone() else {
            return IotaAddress::default(); // err
        };

        #[allow(unreachable_patterns)]
        match transaction.data {
            super::IotaTransactionBlockData::V1(iota_transaction_block_data_v1) => {
                iota_transaction_block_data_v1.sender
            }
            _ => IotaAddress::default(), // err
        }
    }

    pub fn gas(&self) -> i128 {
        let Some(changes) = self.tx.balance_changes.as_ref() else {
            return 0; // err
        };

        changes.iter().map(|c| c.amount).sum()
    }

    pub fn receiver(&self) -> IotaAddress {
        let Some(changes) = self.tx.balance_changes.as_ref() else {
            return IotaAddress::default(); // err
        };

        // balance_changes.len(1) => self to self
        if changes.len() == 1 {
            return self.sender();
        }

        // if len() > 1 ?
        //      when amount < 0 -> sender
        //      when amount > 0 -> receiver
        //      = 0 ???
        let o = changes.iter().find(|c| c.amount > 0).map(|c| c.owner);
        let Some(owner) = o else {
            return IotaAddress::default(); // err
        };

        match owner {
            super::Owner::AddressOwner(iota_address) => iota_address,
            _ => IotaAddress::default(),
        }
    }

    pub fn status(&self) -> WalletTxStatus {
        match self.tx.effects.as_ref().map(|effects| match effects {
            IotaTransactionBlockEffects::V1(inner) => inner.status.is_ok(),
        }) {
            Some(true) => WalletTxStatus::Confirmed,
            Some(false) => WalletTxStatus::Conflicting,
            None => WalletTxStatus::Pending,
        }
    }

    pub fn date(&self) -> String {
        // The timestamp is in milliseconds but we make it into a human-readable format
        self.tx
            .timestamp_ms
            .and_then(|n| Utc.timestamp_millis_opt(n as i64).single())
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default() // default is going to be an empty String here
    }
}

fn extract_u64(input: &IotaCallArg) -> Option<u64> {
    #[allow(unreachable_patterns)]
    match input {
        IotaCallArg::Pure(pure_val) => match pure_val.decode() {
            Some(IotaPureDecodedValue::U64(n)) => Some(n),
            _ => None,
        },
        _ => None,
    }
}
