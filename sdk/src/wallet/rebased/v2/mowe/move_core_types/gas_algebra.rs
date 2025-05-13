// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    convert::From,
    fmt::{self, Debug, Display},
    marker::PhantomData,
    ops::{Add, AddAssign, Mul},
};

use serde::{Deserialize, Serialize};

/// ****************************************************************************
/// ********************* Gas Quantities
///
/// ****************************************************************************
/// *******************
/// An opaque representation of a certain quantity, with the unit being encoded
/// in the type. This type implements checked addition and subtraction, and only
/// permits type-safe multiplication.
#[derive(Serialize, Deserialize)]
pub struct GasQuantity<U> {
    val: u64,
    phantom: PhantomData<U>,
}

impl<U> GasQuantity<U> {
    pub const fn new(val: u64) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0)
    }

    pub const fn one() -> Self {
        Self::new(1)
    }

    pub const fn is_zero(&self) -> bool {
        self.val == 0
    }
}

/// Unit of abstract memory usage in the Move VM.
pub enum AbstractMemoryUnit {}

/// An abstract measurement of the memory footprint of some Move concept (e.g.
/// value, type etc.) in the Move VM.
///
/// This is a legacy concept that is not well defined and will be deprecated
/// very soon. New applications should not be using this.
pub type AbstractMemorySize = GasQuantity<AbstractMemoryUnit>;

/// ****************************************************************************
/// ********************* Units of Measurement
///
/// ****************************************************************************
/// *******************
/// Unit of internal gas.
pub enum InternalGasUnit {}

pub type InternalGas = GasQuantity<InternalGasUnit>;

impl<U> Debug for GasQuantity<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.val, std::any::type_name::<U>())
    }
}

/// ****************************************************************************
/// ********************* Clone & Copy
///
/// ****************************************************************************
/// *******************
#[allow(clippy::non_canonical_clone_impl)]
impl<U> Clone for GasQuantity<U> {
    fn clone(&self) -> Self {
        Self::new(self.val)
    }
}

impl<U> Copy for GasQuantity<U> {}
