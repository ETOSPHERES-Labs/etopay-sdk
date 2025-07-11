use std::f64;

use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum CryptoAmountError {
    #[error("NegativeAmount")]
    NegativeAmount,

    #[error("Decimal: {0}")]
    Decimal(rust_decimal::Error),
}

impl From<rust_decimal::Error> for CryptoAmountError {
    fn from(value: rust_decimal::Error) -> Self {
        Self::Decimal(value)
    }
}

/// A non-negative decimal value. Used as inputs to create purchases or sending a transaction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CryptoAmount(rust_decimal::Decimal);

impl CryptoAmount {
    /// The value of ZERO
    pub const ZERO: Self = Self(rust_decimal::Decimal::ZERO);

    /// Get the inner value of the amount
    pub fn inner(&self) -> rust_decimal::Decimal {
        self.0
    }

    /// Internal helper function to create values in consts during tests.
    ///
    /// # Safety
    ///
    /// This is unsafe since it does not perform any non-negativity checks. Only safe to call if
    /// it is known that the value is non-negative.
    pub const unsafe fn new_unchecked(value: rust_decimal::Decimal) -> Self {
        Self(value)
    }

    pub fn to_f64_lossy(&self) -> f64 {
        // Looking at implementation of to_f64, it always returns Some() although the value may be truncated.
        if let Some(v) = self.0.to_f64() {
            return v;
        }

        log::error!(
            "[unreachable] Could not convert CryptoAmount to f64! {:?}. Please file a bug report!",
            self
        );

        f64::NAN
    }
}

// From u64 is always possible and will yield a Non-negative value
impl From<u64> for CryptoAmount {
    fn from(value: u64) -> Self {
        Self(rust_decimal::Decimal::from(value))
    }
}

impl TryFrom<f64> for CryptoAmount {
    type Error = CryptoAmountError;

    fn try_from(value: f64) -> std::result::Result<Self, Self::Error> {
        Self::try_from(rust_decimal::Decimal::try_from(value)?)
    }
}

impl TryFrom<rust_decimal::Decimal> for CryptoAmount {
    type Error = CryptoAmountError;

    fn try_from(value: rust_decimal::Decimal) -> std::result::Result<Self, Self::Error> {
        if value < rust_decimal::Decimal::ZERO {
            return Err(CryptoAmountError::NegativeAmount);
        }
        Ok(Self(value))
    }
}
impl TryFrom<api_types::api::decimal::Decimal> for CryptoAmount {
    type Error = CryptoAmountError;

    fn try_from(value: api_types::api::decimal::Decimal) -> std::result::Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

impl From<CryptoAmount> for api_types::api::decimal::Decimal {
    fn from(val: CryptoAmount) -> Self {
        Self(val.0)
    }
}

impl TryFrom<CryptoAmount> for f64 {
    type Error = CryptoAmountError;

    fn try_from(value: CryptoAmount) -> std::result::Result<Self, Self::Error> {
        Ok(value.0.try_into()?)
    }
}

// Adding two NonNegativeAmounts will always result in a positive value so this is safe
impl std::ops::Add for CryptoAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// Dividing two NonNegativeAmounts will always result in a positive value so this is safe
impl std::ops::Div for CryptoAmount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

// Multiplying two NonNegativeAmounts will always result in a positive value so this is safe
impl std::ops::Mul for CryptoAmount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rust_decimal_macros::dec;

    #[rstest::rstest]
    #[case(dec!(0.0), Some(CryptoAmount(dec!(0.0))))]
    #[case(dec!(-0.0), Some(CryptoAmount(dec!(-0.0))))]
    #[case(dec!(-1.0), None)]
    #[case(dec!(-10.5), None)]
    #[case(dec!(1.0), Some(CryptoAmount(dec!(1.0))))]
    #[case(dec!(10.0), Some(CryptoAmount(dec!(10.0))))]
    fn test_try_from_non_zero_dec(#[case] value: rust_decimal::Decimal, #[case] expected_value: Option<CryptoAmount>) {
        let amount = CryptoAmount::try_from(value);

        match (amount, expected_value) {
            (Ok(amount), Some(expected)) => assert_eq!(amount, expected),
            (Err(error), None) => assert!(matches!(error, CryptoAmountError::NegativeAmount)),
            (amount, expected) => panic!("expected {expected:?} but got {amount:?} for {value}"),
        }
    }

    #[rstest::rstest]
    #[case(0.0, Some(CryptoAmount(dec!(0.0))))]
    #[case(-0.0, Some(CryptoAmount(dec!(0.0))))] // this is apparently also "negative"
    #[case(-1.0, None)]
    #[case(-10.5, None)]
    #[case(1.0, Some(CryptoAmount(dec!(1.0))))]
    #[case(10.0, Some(CryptoAmount(dec!(10.0))))]
    fn test_try_from_non_zero_f64(#[case] value: f64, #[case] expected_value: Option<CryptoAmount>) {
        let amount = CryptoAmount::try_from(value);

        match (amount, expected_value) {
            (Ok(amount), Some(expected)) => assert_eq!(amount, expected),
            (Err(error), None) => assert!(matches!(error, CryptoAmountError::NegativeAmount)),
            (amount, expected) => panic!("expected {expected:?} but got {amount:?} for {value}"),
        }
    }
}
