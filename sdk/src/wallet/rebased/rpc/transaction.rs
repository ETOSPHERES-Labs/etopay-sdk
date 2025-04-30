// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::super::TransactionDigest;
use super::super::bigint::BigInt;
use super::super::encoding::Base64;
use super::ExecuteTransactionRequestType;
use crate::wallet::rebased::BalanceChange;
use crate::wallet::rebased::CheckpointSequenceNumber;
use crate::wallet::rebased::IotaEvent;
use crate::wallet::rebased::IotaTransactionBlockData;
use crate::wallet::rebased::IotaTransactionBlockEffects;
use crate::wallet::rebased::ObjectChange;
use crate::wallet::rebased::TransactionEvents;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", rename = "TransactionBlockResponse")]
pub struct IotaTransactionBlockResponse {
    pub digest: TransactionDigest,
    /// Transaction input data
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub transaction: Option<IotaTransactionBlock>,
    /// BCS encoded [SenderSignedData] that includes input object references
    /// returns empty array if `show_raw_transaction` is false
    #[serde_as(as = "Base64")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub raw_transaction: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<IotaTransactionBlockEffects>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub events: Option<IotaTransactionBlockEvents>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub object_changes: Option<Vec<ObjectChange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_changes: Option<Vec<BalanceChange>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub timestamp_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed_local_execution: Option<bool>,
    /// The checkpoint number when this transaction was included and hence
    /// finalized. This is only returned in the read api, not in the
    /// transaction execution api.
    #[serde_as(as = "Option<BigInt<u64>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<CheckpointSequenceNumber>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub raw_effects: Vec<u8>,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRunTransactionBlockResponse {
    pub effects: IotaTransactionBlockEffects,
    pub events: IotaTransactionBlockEvents,
    pub object_changes: Vec<ObjectChange>,
    pub balance_changes: Vec<BalanceChange>,
    pub input: IotaTransactionBlockData,
}

#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename = "TransactionBlockEvents", transparent)]
pub struct IotaTransactionBlockEvents {
    pub data: Vec<IotaEvent>,
}

pub type IotaResult<T = ()> = Result<T, crate::Error>;

impl IotaTransactionBlockEvents {
    pub fn try_from(
        events: TransactionEvents,
        tx_digest: TransactionDigest,
        timestamp_ms: Option<u64>,
        resolver: &mut dyn LayoutResolver,
    ) -> IotaResult<Self> {
        Ok(Self {
            data: events
                .data
                .into_iter()
                .enumerate()
                .map(|(seq, event)| {
                    let layout = resolver.get_annotated_layout(&event.type_)?;
                    IotaEvent::try_from(event, tx_digest, seq as u64, timestamp_ms, layout)
                })
                .collect::<Result<_, _>>()?,
        })
    }

    // TODO: this is only called from the indexer. Remove this once indexer moves to
    // its own resolver.
    pub fn try_from_using_module_resolver(
        events: TransactionEvents,
        tx_digest: TransactionDigest,
        timestamp_ms: Option<u64>,
        resolver: &impl GetModule,
    ) -> IotaResult<Self> {
        Ok(Self {
            data: events
                .data
                .into_iter()
                .enumerate()
                .map(|(seq, event)| {
                    let layout = get_layout_from_struct_tag(event.type_.clone(), resolver)?;
                    IotaEvent::try_from(event, tx_digest, seq as u64, timestamp_ms, layout)
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Display for IotaTransactionBlockEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // if self.data.is_empty() {
        //     writeln!(f, "╭─────────────────────────────╮")?;
        //     writeln!(f, "│ No transaction block events │")?;
        //     writeln!(f, "╰─────────────────────────────╯")
        // } else {
        //     let mut builder = TableBuilder::default();

        //     for event in &self.data {
        //         builder.push_record(vec![format!("{event}")]);
        //     }

        //     let mut table = builder.build();
        //     table.with(TablePanel::header("Transaction Block Events"));
        //     table.with(
        //         TableStyle::rounded().horizontals([HorizontalLine::new(1, TableStyle::modern().get_horizontal())]),
        //     );
        //     write!(f, "{table}")
        // }
        write!(f, "@IotaTransactionBlockEvents->Display()")
    }
}

// from iota_transaction.rs
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase", rename = "TransactionBlockResponseOptions", default)]
pub struct IotaTransactionBlockResponseOptions {
    /// Whether to show transaction input data. Default to be False
    pub show_input: bool,
    /// Whether to show bcs-encoded transaction input data
    pub show_raw_input: bool,
    /// Whether to show transaction effects. Default to be False
    pub show_effects: bool,
    /// Whether to show transaction events. Default to be False
    pub show_events: bool,
    /// Whether to show object_changes. Default to be False
    pub show_object_changes: bool,
    /// Whether to show balance_changes. Default to be False
    pub show_balance_changes: bool,
    /// Whether to show raw transaction effects. Default to be False
    pub show_raw_effects: bool,
}

impl IotaTransactionBlockResponseOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn full_content() -> Self {
        Self {
            show_effects: true,
            show_input: true,
            show_raw_input: true,
            show_events: true,
            show_object_changes: true,
            show_balance_changes: true,
            // This field is added for graphql execution. We keep it false here
            // so current users of `full_content` will not get raw effects unexpectedly.
            show_raw_effects: false,
        }
    }

    pub fn with_input(mut self) -> Self {
        self.show_input = true;
        self
    }

    pub fn with_raw_input(mut self) -> Self {
        self.show_raw_input = true;
        self
    }

    pub fn with_effects(mut self) -> Self {
        self.show_effects = true;
        self
    }

    pub fn with_events(mut self) -> Self {
        self.show_events = true;
        self
    }

    pub fn with_balance_changes(mut self) -> Self {
        self.show_balance_changes = true;
        self
    }

    pub fn with_object_changes(mut self) -> Self {
        self.show_object_changes = true;
        self
    }

    pub fn with_raw_effects(mut self) -> Self {
        self.show_raw_effects = true;
        self
    }

    /// default to return `WaitForEffectsCert` unless some options require
    /// local execution
    pub fn default_execution_request_type(&self) -> ExecuteTransactionRequestType {
        // if people want effects or events, they typically want to wait for local
        // execution
        if self.require_effects() {
            ExecuteTransactionRequestType::WaitForLocalExecution
        } else {
            ExecuteTransactionRequestType::WaitForEffectsCert
        }
    }

    pub fn require_input(&self) -> bool {
        self.show_input || self.show_raw_input || self.show_object_changes
    }

    pub fn require_effects(&self) -> bool {
        self.show_effects
            || self.show_events
            || self.show_balance_changes
            || self.show_object_changes
            || self.show_raw_effects
    }

    pub fn only_digest(&self) -> bool {
        self == &Self::default()
    }
}
