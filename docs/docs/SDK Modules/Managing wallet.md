# Wallet management

The SDK provides users with the opportunity to host their own wallets on their personal end-devices in a safe and easy manner. Before discussing wallet management, some information on wallets and what they are is needed to understand how to manage non-custodial hot wallets.

## Hot Wallets: The Swift Side of Crypto

Picture a hot wallet as the bustling city centre of your digital finances. Hot wallets are online, connected to the internet, and readily available for transactions. They provide users with quick access to their cryptocurrencies, making them ideal for active trading and daily transactions. Think of them as your go-to pocket wallet for everyday spending in the digital realm.

However, convenience comes at a cost. The very connectivity that makes hot wallets user-friendly also renders them more vulnerable to cyber threats. Hacking attempts and online attacks pose a constant risk, making it crucial for users to exercise caution and implement additional security measures when relying on hot wallets.


## Wallet Storage

The wallet internally uses a [Shamir's Secret Sharing (SSS)](https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing) algorithm to securely store and retrieve the mnemonic of the user's wallet. The mnemonic entropy (as defined in [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) using the [english wordlist](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)) used to derive the private key is split into three shares, of which two are needed to recover the actual mnemonic. The shares are then stored in three different places (in order of priority):

1. _Local Share_: Stored on the device in a file, or in the browser local storage if on the web. It never leaves the device.
2. _Recovery Share_: Stored in the OAuth provider, currently in Keycloak, and accessed through the ETOPay backend. Also stored in-memory and accessible in the SDK for use in a situation where the wallet is used offline.
3. _Backup Share_: Stored in the ETOPay backend. Encrypted with the user's chosen password (see below for more information about the password).

This setup brings the following benefits:

- The ETOPay backend can never recover the mnemonic since one of the two accessible shares is encrypted with the user's password.
- If access to the ETOPay backend is lost, the user can still recover their wallet using the _Local_ and _Recovery_ shares, provided that they have had the opportunity to store the latter when setting up their wallet.
- Similarly, in the case where the wallet is used offline, the _Local_ and _Recovery_ shares are all that is needed to reconstruct the wallet mnemmonic.


Before any wallet interaction can be performed, the shares need to be collected and, if necessary, decrypted. Therefore, the user has to provide their `pin` for all wallet operations. If the wallet is operating offline, the _Recovery Share_ also need to be set manually (see [`get_recovery_share`](../SDK%20Reference/SDK%20API%20Reference.md#get-recovery-share) and [`set_recovery_share`](../SDK%20Reference/SDK%20API%20Reference.md#set-recovery-share)).
If a share, password or pin is missing when trying to combine the shares and interact with the wallet, an error is returned. See [Example 22. Initialize Wallet from Shares](../SDK%20Examples/Examples.md#22-initialize-wallet-from-shares) for more information on how to handle those errors correctly. After recombining the shares, the _Local Share_ and the in-memory _Recovery Share_ are automatically recreated. Thus the next time the mnemonic is recreated, two shares are already present locally and no request to the backend is needed.


## Networks

The wallet uses the concept of _Networks_ to select which blockchain to interact with. The ETOPay SDK currently supports interacting with the following protocols:

- [Stardust](https://wiki.iota.org/learn/protocols/stardust/introduction/)
- [Ethereum Virtual Machine (EVM)](https://ethereum.org/en/developers/docs/evm/)
- [ERC-20 Smart Contracts](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/) running on EVM.
- Upcoming: [IOTA Rebased](https://blog.iota.org/iota-rebased-fast-forward/) using [Move](https://sui.io/move).

Before any interaction with the wallet and a network can be done, a network need to be selected. This is done using the [`set_network`](../SDK%20Reference/SDK%20API%20Reference.md#set-network) method, which takes the network's unique `key` as a parameter. The list of networks can be fetched from the backend using [`get_networks`](../SDK%20Reference/SDK%20API%20Reference.md#get-supported-networks) to for example allow the end user to select which network they want to interact with.


## Transaction Storage

Due to the nature of blockains, and to avoid having to scan all the historical blocks, the SDK needs to locally keep track of all the transactions a user has performed. Therefore the wallet has a local database, which also stores all user information, that holds this information. This database is stored in a file on compatible platforms and in the browser local storage if used on the web. To configure the location of this database, please see [Configuring the SDK](../SDK%20Configuration/Configuration.md#configuring-the-storage-path-prefix).


## Wallet Lifecycle

### Pin and Password

To securely store the mnemonic one of the shares is encrypted. This could cause usability issues if the user is required to provide this password, which generally need to meet certain standards of length and complexity, for every interaction with the wallet. Thus the SDK is designed to use both a `password` and a `pin`. Before any wallet operations can be performed (eg. creating a new wallet or calling a function on an existing wallet) the user needs to set this password (see [`set_wallet_password`](../SDK%20Reference/SDK%20API%20Reference.md#set-wallet-password) and [`is_wallet_password_set`](../SDK%20Reference/SDK%20API%20Reference.md#is-wallet-password-set)), which is used to encrypt the shares, together with a pin. The password is encrypted using the pin and stored locally in the users profile.

With this setup, every operation with the wallet where a password is needed to recombine the shares only requires the user to enter and remember the pin. This solves the problem of user experience, which can be further improved by using the secure storage available on some platforms such as biometric or facial recognition authentication for storing the pin.

Furthermore there are a few methods for working with the password and pin combination:

- Use [`change_pin`](../SDK%20Reference/SDK%20API%20Reference.md#reset-pin) to change the pin of the password.
- Use [`verify_pin`](../SDK%20Reference/SDK%20API%20Reference.md#verify-pin) to verify correctnes of the pin.
- Use [`set_wallet_password`](../SDK%20Reference/SDK%20API%20Reference.md#set-wallet-password) to set the initial password and to change the password (in which case this will recreate and re-encrypt the shares if they already exist).
- Use [`is_wallet_password_set`](../SDK%20Reference/SDK%20API%20Reference.md#is-wallet-password-set) to check if the password has already been set.

See the examples for more details:

- [Example 10. Verify Pin](../SDK%20Examples/Examples.md#10-verify-pin)
- [Example 11. Reset Pin](../SDK%20Examples/Examples.md#11-reset-pin)
- [Example 12. Change Password](../SDK%20Examples/Examples.md#12-change-password)


### Creating a wallet

To create a new wallet (if no existing shares exist, for example). The user first needs to set a password and pin combination (see [`set-wallet-password`](../SDK%20Reference/SDK%20API%20Reference.md#set-wallet-password)).
A new wallet can then be created in multiple ways, which all generate and uploads the shares and thus requires the pin to be provided:

1. Create a new random wallet ([`create_wallet_from_new_mnemonic`](../SDK%20Reference/SDK%20API%20Reference.md#create-new-wallet)). This generates a random mnemonic that is then returned to the user for backup purposes.
2. Create a wallet from an existing mnemonic ([`create_wallet_from_existing_mnemonic`](../SDK%20Reference/SDK%20API%20Reference.md#create-new-wallet-from-mnemonic)). This takes a user-provided mnemonic and creates a wallet. See [Example 4. Migrate Wallet From Mnemonic](../SDK%20Examples/Examples.md#4-migrate-wallet-from-mnemonic).
3. Create a wallet from a backup ([`create_wallet_from_backup`](../SDK%20Reference/SDK%20API%20Reference.md#create-new-wallet-from-backup)). This takes the bytes of the backup file and the backup password and creates a wallet. A backup can be created with [`create_wallet_backup`](../SDK%20Reference/SDK%20API%20Reference.md#create-a-wallet-backup). See [Example 5. Migrate Wallet From Backup](../SDK%20Examples/Examples.md#5-migrate-wallet-from-backup).

???+ warning
    In case a wallet is created from a new random mnemonic, make sure that the end user has the opportunity to make a copy of it. You can let the user re-enter the mnemonic and validate it with [`verify_mnemonic`](../SDK%20Reference/SDK%20API%20Reference.md#verify-mnemonic) before allowing the user to continue using the wallet.

???+ warning
    Creating a new wallet will overwrite the old one (if there was any). This could cause loss of funds if not done carefully.

### Creating and restoring wallet backups

The SDK provides functionality to create a backup file in `kdbx` format as a byte array. Backups can only be created if a wallet exists.

Create a backup using [`create_wallet_backup`](../SDK%20Reference/SDK%20API%20Reference.md#create-a-wallet-backup) and provide the following:

* `pin`: This is the same PIN that was set for the wallet. Needed to combine the shares into the mnemonic to backup.
* `backup_password`: A new, separate password set specifically for securing the backup file. This is not the same password as the one used for the wallet.

This call returns an array of bytes which, for example, can we written to a user-provided file path or offered as a download.

To restore the backup, use [`create_wallet_from_backup`](../SDK%20Reference/SDK%20API%20Reference.md#create-new-wallet-from-backup) and provide:

* The bytes returned from the call to create a backup.
* `pin`: the pin set for the currently set password. Used to decrypt the password used to encrypt the shares.
* `backup_password`: the password provided used during the backup process.

This will create shares for the new wallet.

See [Example 5. Migrate Wallet From Backup](../SDK%20Examples/Examples.md#5-migrate-wallet-from-backup) for a complete example of creating and restoring from a backup.


### Deleting the wallet

The wallet can be deleted using [`delete_wallet`](../SDK%20Reference/SDK%20API%20Reference.md#delete-wallet). This is a one-way operation to be used under extreme caution as it could result in permanent loss of funds. Note that, similar to any other wallet operation, deleting the wallet is also a wallet operation and requires the user to enter their pin.



## Using Wallet functions

After a wallet is created, or if the user has already created a wallet before, it can be used by calling any of the relevant methods. See for example:

- [`generate_new_address`](../SDK%20Reference/SDK%20API%20Reference.md#generate-a-new-address)
- [`get_balance`](../SDK%20Reference/SDK%20API%20Reference.md#get-balance)
- [`send_amount`](../SDK%20Reference/SDK%20API%20Reference.md#send-amount)
