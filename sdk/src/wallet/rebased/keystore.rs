use bip32::DerivationPath;
use bip39::{Language, Mnemonic, Seed};
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::hash::{Blake2b256, HashFunction};
use fastcrypto::traits::KeyPair;
use fastcrypto::{
    ed25519::{Ed25519KeyPair, Ed25519PrivateKey},
    traits::ToFromBytes,
};
use serde::Serialize;
use std::collections::BTreeMap;

use super::{Intent, IntentMessage, IotaAddress, Signature};

#[derive(Default)]
pub struct InMemKeystore {
    keys: BTreeMap<IotaAddress, Ed25519KeyPair>,
}
impl InMemKeystore {
    pub fn import_from_mnemonic(
        &mut self,
        phrase: &str,
        derivation_path: DerivationPath,
    ) -> Result<IotaAddress, anyhow::Error> {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, "");

        let indexes = derivation_path.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let derived = slip10_ed25519::derive_ed25519_private_key(seed.as_bytes(), &indexes);
        let sk = Ed25519PrivateKey::from_bytes(&derived).unwrap();

        let kp: Ed25519KeyPair = sk.into();

        let (address, kp) = (kp.public().into(), kp);

        self.keys.insert(address, kp);
        Ok(address)
    }
    pub fn addresses(&self) -> Vec<IotaAddress> {
        self.keys.keys().cloned().collect::<Vec<_>>()
    }

    pub fn sign_secure<T>(&self, address: &IotaAddress, msg: &T, intent: Intent) -> Result<Signature, signature::Error>
    where
        T: Serialize,
    {
        Ok(Signature::new_secure(
            &IntentMessage::new(intent, msg),
            self.keys
                .get(address)
                .ok_or_else(|| signature::Error::from_source(format!("Cannot find key for address: [{address}]")))?,
        ))
    }
}

impl From<&Ed25519PublicKey> for IotaAddress {
    fn from(pk: &Ed25519PublicKey) -> Self {
        let mut hasher = Blake2b256::default();
        hasher.update(pk);
        let g_arr = hasher.finalize();
        IotaAddress(g_arr.digest)
    }
}

// #[serde_as]
// #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, AsRef, AsMut)]
// #[as_ref(forward)]
// #[as_mut(forward)]
// pub struct Ed25519IotaSignature(
//     #[schemars(with = "Base64")]
//     #[serde_as(as = "Readable<Base64, Bytes>")]
//     [u8; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1],
// );
