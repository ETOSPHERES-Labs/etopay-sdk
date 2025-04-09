searchState.loadedDescShard("etopay_sdk", 0, "Welcome to the ETOPay SDK – The Decentralized Payment …\nAlloy transport error\nError occurs in sdk backend (api)\nError occurred while handling bip39 compliant mnemonics\nBlock error\nError for calling a Smart Contract\nError raises if value cannot be converted\nError caused by conversions to/from Decimal and f64\nError raises if the wallet address is empty\nKind of error contained in <code>WalletError</code>\nCould not convert hex to address\nInsufficient balance on wallet\nError occurs is the transaction is invalid\nError occurs is the transaction amount is invalid\nIota client error\nIota wallet error\nErrors related to the kdbx storage\nError creating a LocalSigner from the provided mnemonic\nError raises if authentication token is outdated or invalid\nYou need to set the password before you can initialize the …\nError occurs if password is missing\nError raises if something failed to parse\nError waiting for transaction to be included\nAlloy RPC error\nYou need to set / upload the recovery share before you can …\nError occurred while creating or reconstructing shares\nError for decoding a Smart Contract call\nError raises if transaction does not exist\nError occurs in sdk types\nYou need to use the mnemonic or create a wallet before you …\nUser repository error\nWrapper for wallet errors\nError raises if the feature is not implemented\nError occurs if the wallet is not initialized\nWrong pin or password\nshadow-rs mod\nMain SDK module.\nError handling\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nModule containing code related to the KDBX file format\nHelper macro for bindings to return an error if the …\nExported secrecy crate to use in the bindings\nModule containing code related to the SSS secret sharing …\nTypes module for SDK\nWallet manager This module contains the implementation of …\nThe name of the Git branch that this project was built …\nOperating system and architecture on which the project was …\nThe debug configuration with which the project was built. …\nThe target for this build. This is possibly distinct from …\nThe architecture of the target for this build. This is the …\nThe project build time, formatted in modified ISO 8601 …\nThe project build time, formatted according to RFC 2822 …\nThe project build time, formatted according to RFC 3339 …\nThe cargo version which which the project was built, as …\nA long version string describing the project. The version …\nThe number of commits since the last Git tag on the branch …\nThe time of the Git commit that this project was built …\nThe name of the Git branch that this project was built …\nThe name of the Git branch that this project was built …\nThe full commit hash of the Git commit that this project …\nWhether the Git working tree was clean at the time of …\nThe Git working tree status as a list of files with their …\nThe name of the last Git tag on the branch that this …\nThe project’s description, as determined by the …\nThe project’s full version string, as determined by the …\nThe project’s semver major version, as determined by the …\nThe project’s semver minor version, as determined by the …\nThe project’s semver patch version, as determined by the …\nThe project’s semver pre-release version, as determined …\nThe project name, as determined by the Cargo.toml manifest.\nThe Rustup toolchain with which the project was built. …\nRust version with which the project was built. The version …\nThe short hash of the Git commit that this project was …\nThe name of the Git tag that this project was built from. …\nA long version string describing the project. The version …\nPrints all built-in <code>shadow-rs</code> build constants to standard …\nStruct representing the SDK and its core components …\nReset pin\nConfig module. Configuration for SDK\nConfirm purchase request\ncreate deposit for viviswap user\ncreate detail for viviswap user\nCreate a new user\nCreate purchase request\nCreate a kdbx wallet backup from an existing wallet.\nCreate and store a wallet from an existing kdbx backup file\nCreate and store a wallet from an existing mnemonic\nCreate and store a wallet from a new random mnemonic\ncreate withdrawal for viviswap user\nDefault implementation for SDK\nDelete the currently active user and their wallet\nDelete the currently active wallet\nDrop implementation for SDK\nEstimate gas for sending amount to receiver\nExchange module.\nReturns the argument unchanged.\nGenerates a new receiver address (based on selected …\nGet the balance of the user\nA function that returns a multi-line String containing:\nReturn the current exchange rate.\nGet current iban of viviswap user\nGet case details for postident\nGet current kyc status of viviswap\nGet networks\nGet the user preferred network\nGet purchase details\nGet/download the recovery share.\nGet swap details\nGet the list of swaps for the viviswap user.\nGet transaction list\nGet user entity\nGet the open AMLA KYC questions\nGet the currently open/missing documents for KYC\nwallet transaction\nwallet transaction list\nInitialize an user\nCalls <code>U::from(self)</code>.\nCheck if KYC status is verified.\nCheck if the password to use for wallet operations is set. …\nInitialize an SDK instance from a config\nPostident module. This module includes functions for …\nRefresh access token\nSend amount to receiver address\nSet the <code>Config</code> needed by the SDK\nSet network\nSet networks\nSet the user preferred network\nSet/upload the recovery share.\nSet the answer to an open AMLA KYC question\nSet / upload an open KYC document\nSet KYC identity details\nSet KYC residence details\nSet the password to use for wallet operations. If the …\nShare module. The share module provides functionality for …\nStart kyc verification for postident\nCreate new viviswap user and initialize kyc verification\nSubmit the previously entered partial kyc details for …\nTransaction module.\nUpdate IBAN of viviswap user.\nUpdate the kyc details for viviswap to be submitted\nUpdate case status for postident\nUser module. This module defines methods for interacting …\nVerify the mnemonic by checking if the mnemonic is the …\nVerify pin\nViviswap module. The Sdk struct is responsible for …\nWallet module. This module provides methods for …\nStruct to configure the SDK\nvalue of the X-APP-NAME header used to select the OAuth …\nURL to access the backend.\nReturns the argument unchanged.\nLoad the <code>Config</code> directly from a JSON-formatted <code>String</code> or …\nCalls <code>U::from(self)</code>.\nLog level for filtering which log messages that end up in …\nThe root folder used to access the file system. It is …\nVariant to hold a collection of errors\nError raises if there is an error with viviswap api\nError raises viviswap state is invalid\nError occurs if a filed is missing\nError occurs is viviswap user is not found\nError occurs if viviawap user has an existing state\nError raises if field for kyc verification is invalid\nViviswap related errors\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nthe name of the field which is missing\nError occurs in sdk backend (api)\nError occurs if chain id is not defined in configuration\nError caused by conversions to/from Decimal and f64\nError occurs if the chain id for evm network is missing\nContains the error value\nWrapper type for all SDK errors.\nError from fern logger\nError raises if authentication token is outdated or invalid\nError occurs if the config is missing\nError occurs if the network is missing\nUser provided a negative invalid amount to send or create …\nError occurs if the network is unavailable\nContains the success value\nError raises if something failed to parse\nError raises if chain_id cannot be converted\nA <code>core::result::Result</code> with <code>Error</code> as its error variant.\nError occurs when the config is not initialized or there …\nError occurs in sdk types\nError raises if user is already kyc verified\nError occurs when user is not initialized\nError occurs when the user repository is not initialized\nUser repository error\nViviswap related errors\nError occurs in sdk wallet\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nKdbx storage errors\nWrapper for kdbx storage errors\nKdbx key generation errors\nNot found errors\nKdbx open errors\nKdbx unlock errors\nKdbx write errors\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nload mnemonic from kdbx file\nstore mnemonic in kdbx file\nContains all the shares generated by splitting a secret\nA share that can be used with other <code>Share</code> to construct the …\nError produced when working with <code>Share</code> objects.\nbackup share that is shared with etopay backend, encrypted\nCreates shares from a <code>Mnemonic</code> that can be resolved into a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if the share is encrypted.\nshare to store locally\nReconstruct a <code>Mnemonic</code> from the shares. Can be used to …\nrecovery share that the user should download and store …\nFormat this <code>Share</code> to a string value, returned as a [<code>Secret</code>]…\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference The …\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference A file …\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference …\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference\nExport some <code>api_types</code> for the bindings to reference Orders …\nExport some <code>api_types</code> for the bindings to reference\nGet the raw data bytes of this file\nNew Postident case id\nUsername\nBusiness logic for config sdk module\nErrors related to sdk types\nGet the filename of this file\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct a <code>File</code> from a set of bytes\nThe unique ID of this question.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIndicator if this question allows free text answers.\nThe maximum number of answers (including the free-text …\nThe minumum number of answers (including the free-text …\nNetwork definition\nNewtypes used for sensitive data This module contains …\nA list of available answers that the user can choose from.\nThe question the user has to answer.\nbusiness logic for transaction sdk module\nTry to construct a <code>File</code> from an existing base64-encoded …\nBusiness logic for user sdk module\nbusiness logic for viviswap sdk module\nA non-negative decimal value. Used as inputs to create …\nSupported currencies (mirrors <code>api_types</code> but needed so we …\nEthereum token\nIota token\nThe value of ZERO\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the inner value of the amount\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert this <code>Currency</code> into a <code>SwapPaymentDetailKey</code>\nConvert from String to Currency, used at the API boundary …\nError raises if the access token is empty\nError raises if the password is empty\nError raises if the pin is empty\nContains the error value\nError raises if the currency used is invalid / not …\nError raises if the pin or password is incorrect\nContains the success value\nError raises if the password fails to be encrypted\nA <code>core::result::Result</code> with <code>TypeError</code> as its error variant.\nErrors related to sdk types\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nRepresents a network supported by the wallet\nRepresents an EVM-based network (e.g., Ethereum)\nRepresents and EVM based ERC20 Smart Contract token\nRepresents a Stardust network\nURL of the network’s block explorer\nWhether the network supports purchase transactions\nCoin type, as defined by SLIP-0044 standard\nNumber of decimal places for the network’s currency unit\nDisplay name of the network\nSymbol of the network’s currency\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIf this network is a test network\nUnique key for the network\nList of node URLs for the network\nProtocol used by the network\nchain_id\nchain_id\ncontract address\nSimple wrapper around a non-empty access token that cannot …\nAn encrypted password.\nA non-empty pin used to encrypt the password.\nA salt used in the encryption process. Should be unique …\nA password that is not encrypted and stored as plain text.\nHelper function to get the underlying string, use with …\nHelper function to get access to the inner String. Use …\nDecrypt this password with the provided pin and salt. …\nEncrypt this password with the provided pin and salt.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerate a new random <code>EncryptionSalt</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nHelper function to convert into [<code>secrecy::Secret</code>] using …\nHelper function to convert into [<code>secrecy::Secret</code>] using …\nCreate a new <code>EncryptedPassword</code> from raw bytes.\nTry to construct a new <code>PlainPassword</code> from a <code>String</code>-like …\nTry to construct a new <code>EncryptionPin</code> from a <code>String</code>-like …\nConstruct a new <code>AccessToken</code> from a <code>String</code>, returns an …\nGas estimation (EIP-1559)\nPurchase details\nTransaction info\nTransaction list\nwallet transaction info\nList of wallet transactions\nAmount of transfer\nAmount of transfer\nThe amount to be paid.\nApplication specific metadata attached to the tx\nContains block id\nExchange rate\nCurrency of transfer\nTx creation date, if available\nTx creation date, if available\nUrl of network IOTA/ETH\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe maximum amount of gas that the transaction can consume.\nDescribes type of transaction\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe maximum fee the sender is willing to pay per unit of …\nThe maximum tip the sender is willing to pay to miners (in …\nThe network that the transaction is sent in\nUnique key representing a network\nreceiver of the transaction\nThe receiver of the transaction\netopay reference id for the transaction\nsender of the transaction\nStatus of the transfer\nStatus of the transfer\nThe status of transaction\nThe sender address where the fees goes to.\nThe transaction hash on the network\ntransaction id for particular transaction\nTransactions that happens\nList of transaction info\nStruct to manage the state of the currently active …\nRepresents which kyc method the user uses\nUser use postident for kyc\nKyc process not selected\nStruct for storing a user in the database\nUser use viviswap for kyc\nEncrypted Password\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUser KYC status\nUser KYC Type\nThe local share from the SSS scheme, stored as a string …\nSalt\nUser ID for backend (remove or use for telemetry?)\nUsername for DB\nUsername\nUser Viviswap state\nThe user’s wallet manager that can create a WalletUser …\nUser wallet transactions\non add, just do nothing\namla general verification step\ndocument verification step\ngeneral verification step\nidentity verification step\nStruct for new viviswap user\nThe user is partially verified\npersonal verification step\non add, delete the last one\nresidence verification step\nno verification step (no next verification step available)\nThe user is not verified\nThe user is fully verified\nViviswap iban detail\nViviswap deposit contract details\nViviswap deposit details for FIAT to Crypto Swap\nWhen a Viviswap detail is added, there can be different …\nViviswap kyc status\nThe viviswap partial KYC details consisting of details for …\nViviswap local app state\nViviswap user verification status\nViviswap user verification step\nThe viviswap withdrawal contract information\nViviswap withdrawal details for crypto to FIAT swap\nthe address used in the detail\nThe address of the bank of the beneficiary\nThe name of the beneficiary receiving the SEPA transfer\nThe BIC/SWIFT code for the SEPA transfer\nThe unique UUID of the contract\nThe unique UUID to track the withdrawal contract\nThe country of tax residence of the user\nThe crypto address of viviswap where the crypto swap is to …\nThe current IBAN as a viviswap address detail\nThe date of birth of the user as per his legal documents\nThe deposit address (crypto) where the swap will put the …\nThe deposit address, in this case the IBAN of the user, …\nThe details of the deposit (for the user)\nThe details of the withdrawal\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nfull name of the user\nThe full name of the user as per his legal documents\nThe IBAN of the beneficiary\nthe unique id of the address detail\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIs the user an individual\nIs the user a politically exposed person\nIs the regulatory disclosure confirmed by user\nIs the user a US citizen\nthe status from viviswap, whether the address is verified\nThe monthly swap limit of the user in euros\nThe monthly swap limit in euros\nThe name of the bank of the beneficiary\nThe user’s nationality\nCreates a new viviswap state\nNew function to create the viviswap partial KYC details …\nThe next step in verification\nThe details of the partially verified KYC\nThe supported payment methods of viviswap\nThe reference to be entered by the user in his SEPA bank …\nThe reference used by viviswap for the SEPA transfer\nthe current submission step in the KYC onboarding process …\nUsername of new viviswap user\nThe verification status, either Verified, Unverified or …\nthe user verification status\nthe current verified step in the KYC onboarding process …\nThe id of the unique wallet internal to viviswap\nRepresents borrowing a <code>WalletUser</code> instance with a lifetime …\nCreates a wallet and returns an instance to work upon\nImplementation of <code>WalletManager</code> that uses the SSS schema …\nChanges the password of the existing wallet by …\nChecks if the mnemonic resembled by the shares is the same …\nCreate kdbx backup bytes from shares\nCreate kdbx backup bytes from shares\nCreate shares from a kdbx backup byte stream\nCreate shares from a kdbx backup byte stream\nCreate shares from a mnemonic\nCreate shares from a mnemonic\nGenerate a new mnemonic and create shares. Returns the new …\nGenerate a new mnemonic and create shares. Returns the new …\ndeletes the user’s wallet\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the recovery share\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new <code>WalletManagerImpl</code> from a username.\nThe recovery share that the user should download\nSet the recovery share\nTries to instantiate a <code>WalletUser</code> object from shares …")