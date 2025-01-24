# Architecture

![Cawaena Integration Architecture](Cryptpay_Integration.svg)

## Overview

The architecture shows the involved participants and explains how the integration of a client application works with Cawaena. 

### Cawaena Services

The Cawaena services are a bundle of services, which work together, using various infrastructure components like databases, message queues, caches, etc... to provide the different features as listed [here](Features.md)

These services are maintained, tested and updated by the development team behind this project. The source code for the same is maintained in a closed source repository with the development team.

### Web portal

The developers trying to integrate the Cawaena SaaS product would typically access the web portal to register themselves. After the registration, the developer can requisition a fresh new instance for the application from Cawaena through this portal.

Additionally, the web portal allows the developers to configure their instance according to their application needs. It can be also used to undertake some application specific settings and modify them as well. For example, setting the identity provider urls and access points, which is owned by the application but may be used by Cawaena to authenticate users or setting a webhook receiver endpoint, where a notification server is already listening to Cawaena internal events, to push notifications to the end-user devices, etc...

Currently, on the first request, a staging environment of a fresh Cawaena services instance is deployed. This allows the developers to test their application flow and integration with Cawaena before moving to production. Requesting a production instance is an extra step which needs to be done additionally. This helps the developers to fix a version for their production, while they continue testing new features and releases from Cawaena team on the staging environment, before requesting to update to production.

The infrastructure deploys in a private cloud hosted in the european data center. Deployments in client infrastructure and public clouds are currently not possible, however will be considered in the future, for example with client-owned kubernetes cluster or Azure/AWS depolyments. The Cawaena services are coded in software to be fully platform agnostic. Hence, the deployment architecture does not affect the features and working of the services.

### Integration

The integration can occur for the client in three different ways:

1. REST API: A platform-agnostic, robust API to interact with the Cawaena features and a modern and sleek documentation.

    Within this integration, the Cawaena services are exposed via a REST API. The client developers can directly connect to these services using the Open API specification for the services. A drawback for this approach could be that the requirement on the various flows as expected by the Cawaena services needs to be well understood from the documentation. This approach also works only with custodial wallets, which might increase costs for end-users, as compared to a self-custody wallet.

2. SDK: A variety of multi-stack SDKs working together to allow seamless in-app integration with code examples and instructions on best-practices.

    Here, the client developers can download our SDKs and even modify their behaviour according to their own needs. These SDKs are described in detail and the source code is kept open to allow maximum benefit during integration. Using the SDK gives the advantage to the developers in a way that, the SDK does all the heavy lifting in terms of executing the flows together with the Cawaena service whilst ensuring secure self-custody wallets and user management on end-user devices. This takes away a significant amount of integration effort and allows for a quick time-to-market without needing to understand all particular nuances of Cawaena services.

3. White Label App: A multi-platform white label application with intuitive interfaces for low time to market with fully compliant flows to ensure highest user satisfaction.

    As the white label application builds on top of the SDK, it provides the client developers with complete UI/UX for maximum integration with minimal effort. The only thing the client developers would need is to adjust the corporate identiy, build the application and ship it to their end-users. The white label app ensures that all the flows are correctly executed, with the most lucid user experience from on-boarding to using the features of Cawaena. Being open source again, the white label application can also be customized for own needs and making an entirely new application would be like simply adding a few new layouts.

## SDK Internals

The Cawaena SDK is built in `Rust`. It is primarily an implementation of the various functionalities for managing users, wallets, on-boarding of users through KYC (Know Your Customer) processes, payment methods and listing usage information.

The SDK was designed to support the `Cellpic` application. It is a social media application, which allows monetization of user-generated content. However, in the same principle, any digital data, given that it is authentic and its origin can be verified, can be monetized using the Cawaena ecosystem, which includes the Cawaena infrastructure and the sdk.

The big picture behind Cawaena is a data marketplace. Data processing, silo management and search engine features have been excluded by design from Cawaena to make it a minimal ecosystem for monetization.

### Overview of the SDK functional components

The figure below shows the functional component diagram of the SDK. The core of the SDK is a web3 hot-wallet. This wallet is used to store assets on the local machine running the application built with the SDK. The supporting components like the backend API, user state management and access control logic work for improving ease of use for the end user as well as ensuring correct process flow and state transitions between the Cawaena infrastructure and application.

The binding layer is just a simple 1-to-1 wrapper around the SDK functionalities. This just exports the existing business logic implemented in the SDK in rust to other programming stacks to avoid re-implementation as well as guarantee memory safety natively in code.

The access control section at the bottom shows the input parameters needed from the user/application to authenticate itself against the SDK. For the one-time on-boarding in addition to the `pin` and `access_token` the `username` and `password` is also needed. For regular usage, the `pin`, whenever required and `access_token` is required to ensure smooth handling of operations, including internal function calls to the Cawaena infrastructure and the wallet.

```
+-------------------------------------------------------------------------------+
|                                                                               |
|   +-----------------------------------------------------------------------+   |
|   |                                                                       |   |
|   |                                                                       |   |
|   |   +---------------------------------------------+    +--------------+ |   |
|   |   |                                             |    |              | |   |
|   |   |   +-------------------------------------+   |    |   Backend    | |   |
|   |   |   |                                     |   |    |   API        | |   |
|   |   |   |     +------------------------+      |   |    |              | |   |
|   |   |   |     |                        |      |   |    +--------------+ |   |
|   |   |   |     |                        |      |   |                     |   |
|   |   |   |     |                        |      |   |                     |   |
|   |   |   |     |       IOTA SDK         |      |   |    +--------------+ |   |
|   |   |   |     |                        |      |   |    |              | |   |
|   |   |   |     |       Stronghold       |      |   |    | User         | |   |
|   |   |   |     |         wallet         |      |   |    | State        | |   |
|   |   |   |     |         manager        |      |   |    | Management   | |   |
|   |   |   |     |                        |      |   |    +--------------+ |   |
|   |   |   |     +------------------------+      |   |                     |   |
|   |   |   |                                     |   |                     |   |
|   |   |   |           Wallet Manager            |   |    +--------------+ |   |
|   |   |   +-------------------------------------+   |    |              | |   |
|   |   |                                             |    |   Access     | |   |
|   |   |                Wallet User                  |    |   Control    | |   |
|   |   |                                             |    |   Logic      | |   |
|   |   +---------------------------------------------+    +--------------+ |   |
|   |                         SDK                                           |   |
|   +-----------------------------------------------------------------------+   |
|                           Bindings                                            |
|                                                                               |
|                                                                               |
+--+--------------------------------------+------+----------------------------+-+
   |      Onboarding authentication       |      |   Usage authentication     |
   |                                      |      |                            |
   +--^---------^-------^---------^-------+      +-----^-----------^----------+
      |         |       |         |                    |           |
      |         |       |         |                    |           |
      |         |       |         |                    |           |
   Username  Password  Pin    Access token            Pin      Access token
```