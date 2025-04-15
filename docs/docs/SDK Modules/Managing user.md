# User management

The SDK is designed to allow multiple users working with their own wallets on the same end devices sharing the same storage space. This makes it easy for a single person to have multiple alias users for different purposes and use different wallets for each of them to have a clear separation of risks.

The user initialization is done by two main operations in the SDK.

- [*Creating a new user*](#creating-a-new-user) : This creates a new user. All the properties of the user, like his selected KYC process, his KYC status, his access token for the backend, pin, encrypted password, etc. are set with the default values.

- [*Initializing a user*](#initializing-a-user-and-access-token-refresh) : This function initializes the user for a new session. It also checks that a valid access token has been provided by updating the KYC status of the user from the backend in the SDK internal state.

## Creating a new user

???+ note

    The user should already exist in the OAuth system. However, the SDK associates the local user to the system user only when an access token is provided.

The SDK supports multi-user environments and switching between users is fairly simple. Creating a user in the SDK informs the SDK about the user and allows the SDK to manage the user's state locally, whilst syncing it with the backend periodically. This allows the SDK to be used across multiple devices, and ideally on the same device, on multiple storage paths (see [Configuring the storage path](../../SDK%20Configuration/Configuration/#configuring-the-storage-path)). Depending on the platform the user data is stored either on the file system, in the browser local storage or in-memory. This means that changing the storage path for platforms that store the data on the file system would result in the SDK unknowing the existence of the user and would require to create the user once again.

Creating a new user can be done using the [`create_new_user`](../SDK%20Reference/SDK%20API%20Reference.md#creating-a-new-user) function which takes the `username` input parameter. See also [Example 1. Create New User](../../SDK%20Examples/Examples/#1-create-new-user). The `username` should always match the `preferred_username` claim on the JWT `access_token`, otherwise the SDK would not be able to access the backend services for that user.

???+ tip end

    The application can extract the `preferred_username` information automatically from the JWT claim and set the username directly, instead of asking the user to enter the input. A user might mistype or misunderstand and enter a username which might later not work. This would lead to a bad end-user experience and should be avoided.


## Initializing a user and access token refresh

Before any wallet-related operations or KYC processes can be performed, the user needs be initialized. This allows the SDK to create multiple users and by using the initializing function, only the selected user is activated for the session. Without initializing a user, all operations related to the user would fail or conversely the previously initialized user's session will be used and might cause data leakage or corrupt the state. To protect against this, a corresponding access token must be set using [`refresh_access_token`](../SDK%20Reference/SDK%20API%20Reference.md#refreshing-access-token) before a user can be initialized. An invalid access token will result in failure of the initialization.

The access token brings the following benefits for the SDK:

1. Only the correct user with the username would be initialized. Mismatch would cause an error.
2. The application can only initialize a user after authorization of the actual person, since they would need to share their credentials for creating an access token.
3. Any user whose rights have been revoked, due to misuse reports, would not be able to use the system as the access token would be invalid and generating a new one would not be possible.

!!! warning

    The user management is local to the end devices and deleting the application data, cache, temporary data files, browser local storage, etc. or changing the storage path in the configuration will result in a loss of state and that requires the application to re-create and re-initialize the user.

## Deleting a user

Deleting the user is simply deleting the user entity from the local database, while maintaining entries for other users. The [`delete_user`](../SDK%20Reference/SDK%20API%20Reference.md#delete-user) mthod also calls the backend API to trigger an archiving action for the user. Deleting the user also deletes all the local data files for the user. Since this is a one-way operation, a user is required to enter the pin if they have an existing wallet. If there is no wallet, the pin can be skipped and the user is simply deleted locally and archived in the backend. See [Example 18. Delete User](../../SDK%20Examples/Examples/#18-delete-user) for an example.

!!! danger

    Deleting a user not only deletes the user in the system but also deletes all local files and information from the device. This means, that the wallet is also deleted. Hence, a pin is used to verify if the user wishes to delete all this information. Deletion of a wallet without having a backup file or without the mnemonic is extremely dangerous as it can potentially lead to permanent loss of funds.


## User lifecycle overview

          Username    Refresh access   Username              Pin              
             |            token          |                      |             
             |             |             |                      |             
             |             |             |                      |             
             |             |             |                      |             
        +----v---------+   |     +-------v------+        +------v---------+   
        |              |   |     |              |        |                |   
        |  Create      |   |     | Initialize   |        |     Delete     |   
        |  new         +---v-----> User         +-------->     User       |   
        |  User        |         |              |        |                |   
        +--------------+         +-----+--------+        +----------------+   
    Once                                |
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    Multiple                            |
    Times                   +-----------v-----------------+
                            |  User           Wallet      |
                            |  State          Operations  |
                            |  Change                     |
                            +-----------------------------+
