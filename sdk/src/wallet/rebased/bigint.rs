//! BigInt for Serialization/Deserialization.
//! https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/iota_serde.rs#L244

use std::fmt::Formatter;
use std::ops::Deref;
use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de::Deserializer, ser::Serializer};
use serde_with::{DeserializeAs, DisplayFromStr, SerializeAs, serde_as};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct BigInt<T>(#[serde_as(as = "DisplayFromStr")] T)
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display;

impl<T> SerializeAs<T> for BigInt<T>
where
    T: Display + FromStr + Copy,
    <T as FromStr>::Err: Display,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        BigInt(*value).serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for BigInt<T>
where
    T: Display + FromStr + Copy,
    <T as FromStr>::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(*BigInt::deserialize(deserializer)?)
    }
}

impl<T> From<T> for BigInt<T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display,
{
    fn from(v: T) -> BigInt<T> {
        BigInt(v)
    }
}

impl<T> Deref for BigInt<T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Display for BigInt<T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
