# Configuring the SDK

The ETOPay SDK needs to be configured correctly for secure and functional usage. Misconfiguration might lead to potential information leaks as well as bad end-user experience.


## Static Configuration

The static configuration is provided by passing a JSON formatted string to the SDK using the [`set_config`](../SDK%20Reference/SDK%20API%20Reference.md#set-configuration) function. It has the following format, whose fields are described in the sections below.

```json
{
    "auth_provider": "<authentication provider name>",
    "backend_url": "<valid URL to the backend API>",
    "storage_path": "/path/to/valid/folder",
    "log_level": "info"
}
```

???+ info

    The configuration is usually provided by the [dashboard](https://etopayapp.etospheres.com) when creating a new project.

### Auth Provider

The value of the auth provider is related to the Keycloak realm created for you when you create a new project in the [dashboard](https://etopayapp.etospheres.com).

???+ info

    The control of the client credentials, the flows used to fetch the JWT as well as the entire user management including user registration, email verification and user settings is out of scope for ETOPay backend and SDK. This should be managed by applications using the SDK themselves.


Every time the OAuth client refreshes or fetches a new access token for the user, the access token can be updated in the SDK using the [`refresh_access_token`](../SDK%20Reference/SDK%20API%20Reference.md#refreshing-access-token) function.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config("...").unwrap();

        sdk.refresh_access_token(Some("access_token")).await.unwrap();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();

            try {
                sdk.setConfig("...");
                sdk.refreshAccessToken("accessToken");
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import Foundation
    import ETOPaySdk

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")

    do {
        try await sdk.refreshAccessToken(access_token: "access_token")
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")

    await sdk.refreshAccessToken("access_token");
    ```

!!! warning

    The SDK is not responsible for refreshing the access token. Neither does it have the credentials, nor a way to obtain credentials for refreshing an access token for the user. This is the responsibility of the client application integrating the SDK. The <B>refresh_access_token</B> function should not be confused with <B>refresh_token</B> and should not be passed the value of <B>refresh_token</B>. The function needs the value of a valid <B>access_token</B> as a string.
    For more information about how to obtain an access token using OAuth2, see [^1].

### Configuring the backend

The `backend_url` is the URL to the ETOPay backend system. Depending on if you are using the live or the test instance, the values are the following:

- Live: `https://api-etopay.etospheres.com`
- Test: `https://api-test-etopay.etospheres.com`

### Configuring the storage path prefix

For all platforms, except for when using the TypeScript/Javascript bindings, it is mandatory that the application has access to a file system where it is allowed read and write files and directories and sub-directories. This (existing) folder is specified as the `strorage_path` field in the configuration and accepts both releative and absolute paths. Absolute paths are, however, preferred and recommended.


???+ tip

    It is recommended to use a path where only the application has file system rights. Allowing access to other applications is a potential security risk and may incur loss of funds for end-users.

???+ tip

    For use in Android applications, it is important to extract the path where the app has permissions to create files and directories and use it as the storage path. This is generally something like `\data\data\org.example.app\` if the application package is `org.example.app`.


### Logging in the SDK and validating configuration

Whenever the SDK is configured, the logger is automatically initialized. For all platforms except TypeScript/Javascript, whenever a valid log level is specified in the `log_level` field, the logger is initialized to append log messages to a `etopay_sdk.log` file in the specified `storage_path` folder. The different log levels that can be set for the logger are: `trace`, `debug`, `info`, `warn`, `error` and allow for fine-tuning the amount of log messages that are generated. A value of `off` can also be specified to disable logging completely. It is important and recommended to enable the logger since this information can be exported and analyzed during testing and integration, which can help diagnose any issues.


## Complete example

For a complete example of how to setup and configure the SDK before using any of its module functions, please see [Example 0. Shared Setup Code](../SDK%20Examples/Examples.md#0-shared-setup-code).


[^1]:
    The following information links could be used as reference for OAuth2.0 and OpenID Connect

    [OAuth 2.0 Official Website](https://oauth.net/2/)

    [OAuth 2.0 RFC](https://datatracker.ietf.org/doc/html/rfc6749)

    [OAuth 2.0 Simplified](https://aaronparecki.com/oauth-2-simplified/)

    [OAuth 2.0 Playground](https://developers.google.com/oauthplayground)

    [OpenID Connect Official Website](https://openid.net/connect/)

    [OpenID Connect RFC](https://openid.net/specs/openid-connect-core-1_0.html)

    [OpenID Connect Playground](https://www.openidconnect.net/)

    [OpenID Connect Introduction](https://connect2id.com/learn/openid-connect)
