// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

// SizeOneVec is a wrapper around Vec<T> that enforces the size of the vec to be
// 1. This seems pointless, but it allows us to have fields in protocol messages
// that are current enforced to be of size 1, but might later allow other sizes,
// and to have that constraint enforced in the serialization/deserialization
// layer, instead of requiring manual input validation.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[serde(try_from = "Vec<T>")]
pub struct SizeOneVec<T> {
    e: T,
}

impl<T> SizeOneVec<T> {
    pub fn new(e: T) -> Self {
        Self { e }
    }

    pub fn element(&self) -> &T {
        &self.e
    }

    pub fn element_mut(&mut self) -> &mut T {
        &mut self.e
    }

    pub fn into_inner(self) -> T {
        self.e
    }

    pub fn iter(&self) -> std::iter::Once<&T> {
        std::iter::once(&self.e)
    }
}

impl<T> Serialize for SizeOneVec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1))?;
        seq.serialize_element(&self.e)?;
        seq.end()
    }
}

impl<T> TryFrom<Vec<T>> for SizeOneVec<T> {
    type Error = anyhow::Error;

    fn try_from(mut v: Vec<T>) -> Result<Self, Self::Error> {
        if v.len() != 1 {
            Err(anyhow::anyhow!("Expected a vec of size 1"))
        } else {
            Ok(SizeOneVec { e: v.pop().unwrap() })
        }
    }
}
