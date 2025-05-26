use serde::Deserialize;
use std::time::Duration;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};

pub struct RpcClient {
    pub client: Client,
    pub url: String,
}

pub type RpcResult<T> = Result<T, RebasedError>;

impl std::ops::Deref for RpcClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Deserialize)]
pub struct RpcResponse<T> {
    pub result: T,
}

use super::RebasedError;
const CLIENT_SDK_TYPE_HEADER: &str = "client-sdk-type";
/// The version number of the SDK itself. This can be different from the API
/// version.
const CLIENT_SDK_VERSION_HEADER: &str = "client-sdk-version";
/// The RPC API version that the client is targeting. Different SDK versions may
/// target the same API version.
const CLIENT_TARGET_API_VERSION_HEADER: &str = "client-target-api-version";

impl RpcClient {
    pub async fn new(url: &str, request_timeout: Duration) -> Result<Self, RebasedError> {
        let client_version = "0.13.0-alpha"; // TODO: how to specify this?

        let mut headers = HeaderMap::new();
        headers.insert(
            CLIENT_TARGET_API_VERSION_HEADER,
            // in rust, the client version is the same as the target api version
            HeaderValue::from_static(client_version),
        );
        headers.insert(CLIENT_SDK_VERSION_HEADER, HeaderValue::from_static(client_version));
        headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("rust"));

        let mut http_builder = Client::builder().default_headers(headers);

        #[cfg(not(target_family = "wasm"))]
        {
            http_builder = http_builder.timeout(request_timeout);
        }

        Ok(Self {
            client: http_builder.build()?,
            url: url.to_string(),
        })
    }
}
