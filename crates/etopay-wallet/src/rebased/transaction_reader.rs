use super::{IotaAddress, IotaCallArg, IotaPureDecodedValue, IotaTransactionBlockResponse};

// v2
// 1. amount -> or <- ? transaction.data.transaction.inputs.filter(valueType=u64).sum()
// 2. -> or <-?
//   if result.balanceChanges.len() == 1 then we sent to ouselves so ->
//   if result.balanceChanges.len() > 1 then there were two parties
//      sender is result.transaction.data.sender (is it our address?)
//
// 2 v2 => or we can add extra key "send/receive" and then we dont need to do this <-----------
// 3. amount sign = -> ? - : +
// 4. gas
//   result.balanceChanges().iter().amount.sum()
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
        let Some(changes) = self.tx.balance_changes.clone() else {
            return 0; // err
        };

        changes.into_iter().map(|c| c.amount).sum()
    }

    pub fn sgn(&self, x: i128) -> i8 {
        if x >= 0 { 1 } else { -1 }
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
