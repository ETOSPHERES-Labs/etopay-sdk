use api_types::api::viviswap::payment::ViviPaymentMethodsResponse;
use serde::{Deserialize, Serialize};

/// Struct for new viviswap user
#[derive(Debug, Serialize)]
pub struct NewViviswapUser {
    /// Username of new viviswap user
    pub username: String,
}

/// Viviswap user verification step
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub enum ViviswapVerificationStep {
    /// no verification step (no next verification step available)
    Undefined,
    /// general verification step
    General,
    /// personal verification step
    Personal,
    /// residence verification step
    Residence,
    /// identity verification step
    Identity,
    /// amla general verification step
    Amla,
    /// document verification step
    Documents,
}

impl From<api_types::api::viviswap::kyc::KycStep> for ViviswapVerificationStep {
    fn from(value: api_types::api::viviswap::kyc::KycStep) -> Self {
        match value {
            api_types::api::viviswap::kyc::KycStep::Undefined => Self::Undefined,
            api_types::api::viviswap::kyc::KycStep::General => Self::General,
            api_types::api::viviswap::kyc::KycStep::Personal => Self::Personal,
            api_types::api::viviswap::kyc::KycStep::Identity => Self::Identity,
            api_types::api::viviswap::kyc::KycStep::Residence => Self::Residence,
            api_types::api::viviswap::kyc::KycStep::Amla => Self::Amla,
            api_types::api::viviswap::kyc::KycStep::Document => Self::Documents,
            // undefined means the there is no next verification step available, hence this makes sense
            api_types::api::viviswap::kyc::KycStep::Completed => Self::Undefined,
        }
    }
}

/// Viviswap user verification status
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub enum ViviswapVerificationStatus {
    /// The user is fully verified
    Verified,
    /// The user is not verified
    Unverified,
    /// The user is partially verified
    PartiallyVerified,
}

impl From<api_types::api::viviswap::kyc::KycVerificationStatus> for ViviswapVerificationStatus {
    fn from(value: api_types::api::viviswap::kyc::KycVerificationStatus) -> Self {
        match value {
            api_types::api::viviswap::kyc::KycVerificationStatus::Unverified => Self::Unverified,
            api_types::api::viviswap::kyc::KycVerificationStatus::PartiallyVerified => Self::PartiallyVerified,
            api_types::api::viviswap::kyc::KycVerificationStatus::Verified => Self::Verified,
        }
    }
}

/// Viviswap iban detail
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ViviswapAddressDetail {
    /// the unique id of the address detail
    pub id: String,
    /// the address used in the detail
    pub address: String,
    /// the status from viviswap, whether the address is verified
    pub is_verified: bool,
}

/// Viviswap local app state
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ViviswapState {
    /// The verification status, either Verified, Unverified or PartiallyVerified
    pub verification_status: ViviswapVerificationStatus,
    /// The monthly swap limit of the user in euros
    pub monthly_limit_eur: f32,
    /// The next step in verification
    pub next_verification_step: ViviswapVerificationStep,
    /// The details of the partially verified KYC
    pub partial_kyc_details_input: ViviswapPartiallyKycDetails,
    /// The current IBAN as a viviswap address detail
    pub current_iban: Option<ViviswapAddressDetail>,
    /// The supported payment methods of viviswap
    pub payment_methods: Option<ViviPaymentMethodsResponse>,
}

impl ViviswapState {
    /// Creates a new viviswap state
    pub fn new() -> ViviswapState {
        ViviswapState {
            verification_status: ViviswapVerificationStatus::Unverified,
            monthly_limit_eur: 0.0,
            next_verification_step: ViviswapVerificationStep::General,
            partial_kyc_details_input: ViviswapPartiallyKycDetails::new(),
            current_iban: Option::None,
            payment_methods: Option::None,
        }
    }
}

impl Default for ViviswapState {
    fn default() -> Self {
        Self::new()
    }
}
/// Viviswap kyc status
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ViviswapKycStatus {
    /// full name of the user
    pub full_name: String,
    /// the current submission step in the KYC onboarding process for the user
    pub submission_step: ViviswapVerificationStep,
    /// the current verified step in the KYC onboarding process for the user
    pub verified_step: ViviswapVerificationStep,
    /// the user verification status
    pub verification_status: ViviswapVerificationStatus,
    /// The monthly swap limit in euros
    pub monthly_limit_eur: f32,
}

/// When a Viviswap detail is added, there can be different ways to handle the logic
#[derive(Debug)]
pub enum ViviswapDetailUpdateStrategy {
    /// on add, delete the last one
    Replace,
    /// on add, just do nothing
    Add,
}

/// The viviswap partial KYC details consisting of details for general and personal KYC steps
#[derive(Debug, Default, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub struct ViviswapPartiallyKycDetails {
    /// Is the user an individual
    pub is_individual: Option<bool>,
    /// Is the user a politically exposed person
    pub is_pep: Option<bool>,
    /// Is the user a US citizen
    pub is_us_citizen: Option<bool>,
    /// Is the regulatory disclosure confirmed by user
    pub is_regulatory_disclosure: Option<bool>,
    /// The country of tax residence of the user
    pub country_of_residence: Option<String>,
    /// The user's nationality
    pub nationality: Option<String>,
    /// The full name of the user as per his legal documents
    pub full_name: Option<String>,
    /// The date of birth of the user as per his legal documents
    pub date_of_birth: Option<String>,
}

impl ViviswapPartiallyKycDetails {
    /// New function to create the viviswap partial KYC details with default None
    pub fn new() -> ViviswapPartiallyKycDetails {
        ViviswapPartiallyKycDetails {
            is_individual: Option::None,
            is_pep: Option::None,
            is_us_citizen: Option::None,
            is_regulatory_disclosure: Option::None,
            country_of_residence: Option::None,
            nationality: Option::None,
            full_name: Option::None,
            date_of_birth: Option::None,
        }
    }
}

/// Viviswap deposit details for FIAT to Crypto Swap
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViviswapDepositDetails {
    /// The reference to be entered by the user in his SEPA bank transfer
    pub reference: String,
    /// The name of the beneficiary receiving the SEPA transfer
    pub beneficiary: String,
    /// The name of the bank of the beneficiary
    pub name_of_bank: String,
    /// The address of the bank of the beneficiary
    pub address_of_bank: String,
    /// The IBAN of the beneficiary
    pub iban: String,
    /// The BIC/SWIFT code for the SEPA transfer
    pub bic: String,
}

/// Viviswap deposit contract details
#[derive(Debug, Serialize)]
pub struct ViviswapDeposit {
    /// The unique UUID of the contract
    pub contract_id: String,
    /// The deposit address (crypto) where the swap will put the funds from fiat
    pub deposit_address: String,
    /// The details of the deposit (for the user)
    pub details: ViviswapDepositDetails,
}

/// Viviswap withdrawal details for crypto to FIAT swap
#[derive(Serialize)]
pub struct ViviswapWithdrawalDetails {
    /// The reference used by viviswap for the SEPA transfer
    pub reference: String,
    /// The id of the unique wallet internal to viviswap
    pub wallet_id: String,
    /// The crypto address of viviswap where the crypto swap is to be sent
    pub crypto_address: String,
}

/// The viviswap withdrawal contract information
#[derive(Serialize)]
pub struct ViviswapWithdrawal {
    /// The unique UUID to track the withdrawal contract
    pub contract_id: String,
    /// The deposit address, in this case the IBAN of the user, where fiat will be deposited.
    pub deposit_address: String,
    /// The details of the withdrawal
    pub details: ViviswapWithdrawalDetails,
}
