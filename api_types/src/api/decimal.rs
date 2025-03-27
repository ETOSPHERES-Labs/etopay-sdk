/// Represents a decimal amount of either FIAT or crypto currencies, always in the main unit for
/// respective currency/network (eg. EURO, USD, ETH, IOTA).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct Decimal(pub rust_decimal::Decimal);

impl From<rust_decimal::Decimal> for Decimal {
    fn from(value: rust_decimal::Decimal) -> Self {
        Self(value)
    }
}

/// Implementations of serde Serialize and Deserialize to make sure it is represented correctly
/// as a String.
mod serde {
    use super::Decimal;

    impl serde::Serialize for Decimal {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }

    struct DecimalVisitor;

    impl serde::de::Visitor<'_> for DecimalVisitor {
        type Value = Decimal;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("String containing a decimal number")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            // This is the crucial part: using `from_str_exact` we make sure no rounding is
            // implicitly performed. Instead, there will be an error if the number cannot be
            // represented correctly.
            //
            // The built-in serialization uses `from_str` which applies truncation/rounding to fit
            // which we do not want!
            rust_decimal::Decimal::from_str_exact(v)
                .map(Decimal)
                .map_err(|e| E::custom(format!("Could not parse {v}: {e}")))
        }
    }

    impl<'de> serde::Deserialize<'de> for Decimal {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_string(DecimalVisitor)
        }
    }
}
