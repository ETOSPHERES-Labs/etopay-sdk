use super::error::{Result, TypeError};
use api_types::api::{generic::ApiCryptoCurrency, viviswap::detail::SwapPaymentDetailKey};
use serde::Serialize;

/// Supported currencies (mirrors `api_types` but needed so we can implement the additional
/// `coin_type` and `to_vivi_payment_method_key` function)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Currency {
    /// Iota token
    Iota,
    /// Ethereum token
    Eth,
}

impl TryFrom<String> for Currency {
    type Error = TypeError;
    /// Convert from String to Currency, used at the API boundary to interface with the bindings.
    fn try_from(currency: String) -> Result<Self> {
        match currency.to_lowercase().as_str() {
            "iota" => Ok(Self::Iota),
            "eth" => Ok(Self::Eth),
            _ => Err(TypeError::InvalidCurrency(currency)),
        }
    }
}

impl Currency {
    /// Convert this [`Currency`] into a [`SwapPaymentDetailKey`]
    pub fn to_vivi_payment_method_key(self) -> SwapPaymentDetailKey {
        match self {
            Self::Iota => SwapPaymentDetailKey::Iota,
            Self::Eth => SwapPaymentDetailKey::Eth,
        }
    }
}

/// We want to convert from our internal Currency enum into the one used in the API.
impl From<Currency> for ApiCryptoCurrency {
    fn from(value: Currency) -> Self {
        match value {
            Currency::Iota => ApiCryptoCurrency::Iota,
            Currency::Eth => ApiCryptoCurrency::Eth,
        }
    }
}
impl From<ApiCryptoCurrency> for Currency {
    fn from(value: ApiCryptoCurrency) -> Self {
        match value {
            ApiCryptoCurrency::Iota => Currency::Iota,
            ApiCryptoCurrency::Eth => Currency::Eth,
        }
    }
}

// the display implementation must be compatible with TryFrom<String> since it is part of the
// public binding interface.
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::Iota => write!(f, "Iota"),
            Currency::Eth => write!(f, "Eth"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::currencies::Currency;

    use rust_decimal_macros::dec;

    #[rstest::rstest]
    fn test_display_roundtrip(#[values(Currency::Iota, Currency::Eth)] c: Currency) {
        assert_eq!(c, Currency::try_from(c.to_string()).unwrap());
    }
}
