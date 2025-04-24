// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//

use std::{cell::OnceCell, fmt::Debug};

use serde::{Deserialize, Serialize};

use super::{EmptySignInfo, IntentScope};

pub trait Message {
    type DigestType: Clone + Debug;
    const SCOPE: IntentScope;

    fn scope(&self) -> IntentScope {
        Self::SCOPE
    }

    fn digest(&self) -> Self::DigestType;
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Envelope<T: Message, S> {
    #[serde(skip)]
    digest: OnceCell<T::DigestType>,

    data: T,
    auth_signature: S,
}

impl<T: Message> Envelope<T, EmptySignInfo> {
    pub fn new(data: T) -> Self {
        Self {
            digest: OnceCell::new(),
            data,
            auth_signature: EmptySignInfo {},
        }
    }
}

impl<T: Message, S> Envelope<T, S> {
    pub fn new_from_data_and_sig(data: T, sig: S) -> Self {
        Self {
            digest: Default::default(),
            data,
            auth_signature: sig,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn into_sig(self) -> S {
        self.auth_signature
    }

    pub fn into_data_and_sig(self) -> (T, S) {
        let Self {
            data, auth_signature, ..
        } = self;
        (data, auth_signature)
    }

    pub fn auth_sig(&self) -> &S {
        &self.auth_signature
    }

    pub fn auth_sig_mut_for_testing(&mut self) -> &mut S {
        &mut self.auth_signature
    }

    pub fn digest(&self) -> &T::DigestType {
        self.digest.get_or_init(|| self.data.digest())
    }

    pub fn data_mut_for_testing(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: Message + PartialEq, S: PartialEq> PartialEq for Envelope<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.auth_signature == other.auth_signature
    }
}
