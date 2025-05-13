// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use super::super::hash::{Blake2b256, HashFunction};

/// Something that we know how to hash and sign.
pub trait Signable<W> {
    fn write(&self, writer: &mut W);
}

/// Activate the blanket implementation of `Signable` based on serde and BCS.
/// * We use `serde_name` to extract a seed from the name of structs and enums.
/// * We use `BCS` to generate canonical bytes suitable for hashing and signing.
///
/// # Safety
/// We protect the access to this marker trait through a "sealed trait" pattern:
/// impls must be add added here (nowehre else) which lets us note those impls
/// MUST be on types that comply with the `serde_name` machinery
/// for the below implementations not to panic. One way to check they work is to
/// write a unit test for serialization to / deserialization from signable
/// bytes.
mod bcs_signable {
    use crate::wallet::rebased::TransactionEvents;

    pub trait BcsSignable: serde::Serialize + serde::de::DeserializeOwned {}
    // impl BcsSignable for crate::committee::Committee {}
    // impl BcsSignable for crate::messages_checkpoint::CheckpointSummary {}
    // impl BcsSignable for crate::messages_checkpoint::CheckpointContents {}
    //
    // impl BcsSignable for crate::effects::TransactionEffects {}
    impl BcsSignable for TransactionEvents {}
    impl BcsSignable for super::super::TransactionData {}
    // impl BcsSignable for crate::transaction::SenderSignedData {}
    impl BcsSignable for crate::wallet::rebased::v2::iota::iota_types::object::ObjectInner {}
    //
    // impl BcsSignable for crate::accumulator::Accumulator {}
    //
    // impl BcsSignable for super::bcs_signable_test::Foo {}
    // #[cfg(test)]
    // impl BcsSignable for super::bcs_signable_test::Bar {}
}

impl<T, W> Signable<W> for T
where
    T: bcs_signable::BcsSignable,
    W: std::io::Write,
{
    #[allow(clippy::expect_used, reason = "invariant guaranteed by the sealed trait above")]
    fn write(&self, writer: &mut W) {
        let name = serde_name::trace_name::<Self>().expect("Self must be a struct or an enum");
        // Note: This assumes that names never contain the separator `::`.
        write!(writer, "{}::", name).expect("Hasher should not fail");
        bcs::serialize_into(writer, &self).expect("Message serialization should not fail");
    }
}

fn hash<S: Signable<H>, H: HashFunction<DIGEST_SIZE>, const DIGEST_SIZE: usize>(signable: &S) -> [u8; DIGEST_SIZE] {
    let mut digest = H::default();
    signable.write(&mut digest);
    let hash = digest.finalize();
    hash.into()
}

// pub fn default_hash<S: Signable<Blake2b256>>(signable: &S) -> [u8; 32] {
//     hash::<S, Blake2b256, 32>(signable)
// }

pub type DefaultHash = Blake2b256;

pub fn default_hash<S: Signable<DefaultHash>>(signable: &S) -> [u8; 32] {
    hash::<S, DefaultHash, 32>(signable)
}
