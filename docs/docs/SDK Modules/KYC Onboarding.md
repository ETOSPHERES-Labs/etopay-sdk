# Understanding KYC

## What is KYC?

Know Your Customer (KYC) is a regulatory and compliance process used by financial institutions and businesses to verify the identity of their customers. The primary goal of KYC is to prevent fraudulent activities such as identity theft, financial fraud, and money laundering. Traditionally, KYC involves collecting personal information, such as government-issued identification, proof of address, and other relevant documents, to ensure that individuals engaging in financial transactions are legitimate.

With the rise of blockchain and decentralized finance (DeFi), the importance of KYC has extended beyond traditional banking into the Web3 ecosystem. While the decentralized nature of Web3 promotes anonymity and self-sovereign identity, it also introduces challenges in regulatory compliance, particularly in preventing illicit financial activities.

## The Purpose of KYC in Web3

In the Web3 industry, KYC serves several key purposes:

1. **Preventing Money Laundering** – Cryptocurrencies and blockchain-based financial systems can be exploited by criminals to launder illicit funds due to their pseudonymous nature. Implementing KYC ensures that users engaging in financial transactions are properly identified, reducing the risk of money laundering.

2. **Ensuring Regulatory Compliance** – Governments and regulatory bodies worldwide have imposed Anti-Money Laundering (AML) and Counter-Terrorist Financing (CTF) laws that apply to cryptocurrency exchanges, DeFi platforms, and NFT marketplaces. KYC compliance helps Web3 projects adhere to these regulations and avoid legal repercussions.

3. **Mitigating Fraud and Scams** – The Web3 space has seen numerous scams, including rug pulls, Ponzi schemes, and fraudulent token offerings. By implementing KYC, projects can verify the legitimacy of users and project teams, fostering trust within the ecosystem.

4. **Protecting Institutional Investors** – Institutional investors are increasingly exploring Web3 and DeFi. However, without proper KYC measures, they face significant risks. KYC-compliant platforms provide greater security and legitimacy, encouraging broader institutional participation.

## Onboarding users for KYC

Currently, the ETOPay SDK offers two alternatives for onboarding users through a KYC process. These two processes are indepedent of each other, however the ETOPay SDK consults the backend to check which process the user has started and also continues from the state where the user previously left. hence, in case if the user loses their devices, or re-initiates the onboarding through another client of the ETOPay SDK, the user is guided onward from where they left of, instead of starting from the beginning.

The two KYC processes that are offered by the SDK are namely:

1. Using Postident as the KYC Provider
2. Using ETOSPHERES Exchange (previously known as viviswap) as the KYC Provider.

The onboarding steps in the SDK follow a very simple flow:

1. Check if the user is already KYC verified. This is independent of the process the user followed.
2. If the user is not verified, check if the user has already started a process.
3. If the user as already started a process, continue the process with the same provider as the user has started.
4. Or offer the user a choice and start with the chosen provider.

???+ info

    Currently, the federation of the KYC status of the user across multiple customers is not supported. This would mean that the same user might need to perform a KYC again for a different ETOPay customer. This feature is highly demanded as it improves the user onboarding, and it provides new ETOPay customers immediate access to a verified user pool. However, the feature is currently under scrutiny and is bound to be planned in future releases of the ETOPay platform.

!!! warning

    There is a provider lock, that currently prevents users from switching between the two providers, or in future, potentially multiple providers, once they chose to start the process with a certain provider. Due to this, the user is forced to complete the onboarding with the same provider they chose at the beginning and have no option of switching providers once a process is started.

### Onboarding users via Postident

For onboarding users on KYC via Postident allows users with existing wallets to simply perform a KYC onboarding, migrate the mnemonic to ETOPay and start using the wallet services directly.

Before being able to use the Postident integration in the SDK for all the end-users, the customers should register with Postident and get a business account for using their services. The SDK is merely a wrapper around the Postident Standard Connect and Response (SCR) interface for faster onboarding without the need of programming own integrations. After creating a business account, the Postident provides information as shown in the table below.

There are typically two access credentials provided. One access to their `TestApp` on the ITU can be plugged in to the ETOPay test environment. The other access to their production can be plugged in to the ETOPay production environment.

???+ info

    The Postident also publishes their own SDK for both Android and iOS platforms. These SDKS work in conjunction with the ETOPay SDK. The SDK's from Postident provide the UI/UX for the KYC Onboarding, whilst the ETOPay SDK handles the business logic of generating case Ids, tracking them and securely encrypting. Simply feeding the case Ids generated by the ETOPay SDK to the Postident SDK provides a complete all rounded KYC Integration for end-users with a simplified UI/UX

For any additional help regarding onboarding with Postident as a business customer or questions for integration with Postident, please free to always reach out to our [team](mailto:contact@etospheres.com)

!!! warning

    The JSON Web Encryption(JWE) is always enabled on all endpoints at ETOPay. Hence, after on-boarding with Postident, creating cases intially might fail, if you have not reached out to Postident and informed them to also enable the encryption on their end. This included also enabling encryption for ITU (Test environment of ETOPay). This is default behavior and for security reasons, the JWE will never be disabled, even on ETOPay test environment.

The following table shows the information that is required by the custoemr. By plugging it in the ETOPay application dashboard, it permits ETOPay to connect to the Postident services and enable the KYC onboarding for all SDK clients.

| Parameter | Description|
|:----:|:----:|
| ClientId | This information is used in the ETOPay application for connecting to the Postident SCR. This basically represents the username for login using basic auth for all Postident SCR endpoints. |
| Login password | This information authenticates the above client against all the SCR endpoint and is passed in the header along with the username as basic authentication |
| Data password/Encryption password| This password is used for generating the HMAC of the public key, which is generated and used by the system for JWE payloads.|

### Onboarding users via ETOSPHERES Exchange

Similar to Postident, in order to use the ETOSPHERES Exchange as a process for KYC on-boarding of users, customers are required to register their business account directly with the ETOSPHERES Exchange. Once registered, the customers are able to access credentials listed below in the table, one each for a testing and for a production environment.

| Parameter | Description|
|:----:|:----:|
| Organisation Id | This information identifies the organisation account at the Exchange.  |
| API Key  | This key is used as part of the JWT payload. This key is unique for each organisation and environment |
| API Secret | This secret is used for generating the HMAC-based signature of the JWT Payload. This secret is also unique to each organisation and environment|

For more information on how ETOPay authenticates against the API of the Exchange, please see [here](https://api-service-dev.viviswap.com/docs/#section/Authentication)

Please reach out to us directly for onboarding as a business on the ETOSPHERES Exchange [here](mailto:contact@etospheres.com)

The onboarding of users and their related KYC process is described directly in our [documentation](
https://api-service-dev.viviswap.com/docs/#section/Know-your-customer-(KYC)) hosted at the exchange. The ETOPay SDK completely integrates the organisations API scope. In addition, the existed users on the exchange are also immediately federated to the organisations, once they approve to share their KYC data with the customer attached organisation at the exchange. This feature is however, not yet available directly in ETOPay and the approval for sharing data is only possible right now via an E-Mail from the user.
