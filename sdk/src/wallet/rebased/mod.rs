//! Contains logic for interfacing with the IOTA Rebased Network.
//! This deliberately exposes a minimal set of types / interfaces so that it can easily be
//! moved to a separate crate if we want to in the future. It should not import or use anything
//! from the rest of the sdk crate!
//!

#![allow(clippy::expect_used, clippy::unwrap_used)] // used in some serialize/deserialize locations
#![allow(dead_code)] // TEMP

mod bigint;
mod keystore;
mod rpc;
mod serde;
mod types;

pub use rpc::*;
pub use types::*;

pub use keystore::InMemKeystore;

use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};

pub struct RpcClient {
    pub client: HttpClient,
}

const CLIENT_SDK_TYPE_HEADER: &str = "client-sdk-type";
/// The version number of the SDK itself. This can be different from the API
/// version.
const CLIENT_SDK_VERSION_HEADER: &str = "client-sdk-version";
/// The RPC API version that the client is targeting. Different SDK versions may
/// target the same API version.
const CLIENT_TARGET_API_VERSION_HEADER: &str = "client-target-api-version";

impl RpcClient {
    pub fn new(url: &str) -> Self {
        let client_version = "0.13.0-alpha"; // TODO: how to specify this?

        let mut headers = HeaderMap::new();
        headers.insert(
            CLIENT_TARGET_API_VERSION_HEADER,
            // in rust, the client version is the same as the target api version
            HeaderValue::from_static(client_version),
        );
        headers.insert(CLIENT_SDK_VERSION_HEADER, HeaderValue::from_static(client_version));
        headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("rust"));

        let http_builder = HttpClientBuilder::default()
            .max_request_size(2 << 30)
            .set_headers(headers);
        // .request_timeout(self.request_timeout);

        Self {
            client: http_builder.build(url).expect("could not create client"),
        }
    }
}
