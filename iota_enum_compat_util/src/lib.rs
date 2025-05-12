// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{io::Write, path::PathBuf};

pub trait EnumOrderMap {
    fn order_to_variant_map() -> std::collections::BTreeMap<u64, String>;
}
