use crate::types::WalletTxStatus;

use super::{IotaAddress, TransactionReader};

pub struct Amount {
    pub amount: u128,
    pub sgn: i8,
}

pub struct TransactionFormatter<'a> {
    reader: &'a TransactionReader<'a>,
    active_address: IotaAddress,
}

impl<'a> TransactionFormatter<'a> {
    pub fn new(reader: &'a TransactionReader, active_address: IotaAddress) -> Self {
        Self { reader, active_address }
    }

    pub fn amount(&self) -> Amount {
        // a -> a
        if self.reader.sender() == self.reader.receiver() {
            // in this situation, IOTA wallet shows gas difference because: sent (-1) + received (+1) = 0
            let gas = self.reader.gas();
            let sgn = if gas >= 0 { 1 } else { -1 };

            return Amount {
                amount: gas.unsigned_abs(),
                sgn,
            };
        }

        // a -> b or b -> a
        let sgn = if self.is_sender() { -1 } else { 1 };
        let amount = self.reader.amount();

        Amount {
            amount: u128::from(amount),
            sgn,
        }
    }

    pub fn is_sender(&self) -> bool {
        self.reader.sender() == self.active_address
    }

    pub fn sender(&self) -> IotaAddress {
        self.reader.sender()
    }

    pub fn receiver(&self) -> IotaAddress {
        self.reader.receiver()
    }

    pub fn status(&self) -> WalletTxStatus {
        self.reader.status()
    }

    pub fn date(&self) -> String {
        self.reader.date()
    }
}
