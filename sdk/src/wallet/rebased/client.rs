#[cfg(not(target_arch = "wasm32"))]
use jsonrpsee::http_client::HttpClient as Client;

#[cfg(target_arch = "wasm32")]
use jsonrpsee::wasm_client::Client;

pub struct RpcClient {
    client: Client,
}

impl std::ops::Deref for RpcClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod non_wasm {
    use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClientBuilder};

    const CLIENT_SDK_TYPE_HEADER: &str = "client-sdk-type";
    /// The version number of the SDK itself. This can be different from the API
    /// version.
    const CLIENT_SDK_VERSION_HEADER: &str = "client-sdk-version";
    /// The RPC API version that the client is targeting. Different SDK versions may
    /// target the same API version.
    const CLIENT_TARGET_API_VERSION_HEADER: &str = "client-target-api-version";

    impl super::RpcClient {
        pub async fn new(url: &str) -> Self {
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
}

#[cfg(target_arch = "wasm32")]
impl RpcClient {
    pub async fn new(url: &str) -> Self {
        use jsonrpsee::wasm_client::WasmClientBuilder;
        let http_builder = WasmClientBuilder::default();
        // .request_timeout(self.request_timeout);

        Self {
            client: http_builder.build(url).await.expect("could not create client"),
        }
    }
}
