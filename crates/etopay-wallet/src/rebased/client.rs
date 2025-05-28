#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;

pub struct RpcClient {
    pub client: Client,
    pub url: String,
}

pub type RpcResult<T> = Result<T, RebasedError>;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RawRpcResponse<T> {
    Success { result: T },
    Error { error: RpcError },
}

#[derive(Deserialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

impl<T> RawRpcResponse<T> {
    pub fn into_result(self) -> RpcResult<T> {
        match self {
            RawRpcResponse::Success { result } => Ok(result),
            RawRpcResponse::Error { error } => match error.code {
                -32602 => Err(RebasedError::TransactionNotFound),
                code => Err(RebasedError::RpcCodeAndMessage(code, error.message)),
            },
        }
    }
}

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
    pub async fn new(url: &str) -> Result<Self, RebasedError> {
        let client_version = "0.13.0-alpha"; // TODO: how to specify this?

        let mut headers = HeaderMap::new();
        headers.insert(
            CLIENT_TARGET_API_VERSION_HEADER,
            // in rust, the client version is the same as the target api version
            HeaderValue::from_static(client_version),
        );
        headers.insert(CLIENT_SDK_VERSION_HEADER, HeaderValue::from_static(client_version));
        headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("rust"));

        #[cfg(not(target_family = "wasm"))]
        let http_builder = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10));
        #[cfg(target_family = "wasm")]
        let http_builder = Client::builder().default_headers(headers);

        Ok(Self {
            client: http_builder.build()?,
            url: url.to_string(),
        })
    }
}
