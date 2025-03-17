use crate::backend;
use crate::types::viviswap::{
    ViviswapKycStatus, ViviswapPartiallyKycDetails, ViviswapState, ViviswapVerificationStatus, ViviswapVerificationStep,
};
use crate::{
    backend::viviswap::{
        create_viviswap_user, get_viviswap_kyc_status, set_viviswap_kyc_general_details,
        set_viviswap_kyc_personal_details,
    },
    types::viviswap::NewViviswapUser,
};
use crate::{
    core::{viviswap::ViviswapError, Sdk},
    error::Result,
};
use api_types::api::viviswap::kyc::{
    File, IdentityOfficialDocumentData, IdentityPersonalDocumentData, KycAmlaQuestion, KycOpenDocument, KycStep,
};
use chrono::{NaiveDate, Utc};
use log::*;

impl Sdk {
    /// Create new viviswap user and initialize kyc verification
    ///
    /// # Arguments
    ///
    /// * `mail` - The email address of the user.
    /// * `terms_accepted` - A boolean indicating whether the terms have been accepted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `NewViviswapUser` if successful, or a [`crate::Error`] if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of [`crate::Error`] if any of the following conditions are met:
    ///
    /// * Repository initialization error.
    /// * User already exists.
    /// * Viviswap API error.
    /// * User status update error.
    pub async fn start_kyc_verification_for_viviswap(
        &mut self,
        mail: &str,
        terms_accepted: bool,
    ) -> Result<NewViviswapUser> {
        info!("Starting KYC verification with viviswap");
        // load user entity
        let mut user = self.get_user().await?;

        // load repository
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        // check if user has already a viviswap state available
        if user.viviswap_state.is_some() {
            return Err(crate::Error::Viviswap(ViviswapError::UserStateExisting));
        };

        let new_viviswap_user = create_viviswap_user(config, access_token, mail, terms_accepted).await?;

        user.viviswap_state = Some(ViviswapState::new());

        repo.update(&user)?;

        Ok(NewViviswapUser {
            username: new_viviswap_user.username,
        })
    }

    /// Get current kyc status of viviswap
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ViviswapKycStatus` if successful, or a [`crate::Error`] if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of [`crate::Error`] if any of the following conditions are met:
    ///
    /// * Repository initialization error.
    /// * Viviswap API error.
    pub async fn get_kyc_details_for_viviswap(&mut self) -> Result<ViviswapKycStatus> {
        info!("Getting KYC details for viviswap user");
        // load user entity
        let user = self.get_user().await?;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let kyc_status = get_viviswap_kyc_status(config, access_token).await?;
        debug!("KYC Status response: {kyc_status:#?}");

        let username = user.username;
        let verification_status: ViviswapVerificationStatus = kyc_status.verification_status.into();

        let monthly_limit_eur = kyc_status.monthly_limit_eur;
        let next_verification_step = match kyc_status.verified_step {
            KycStep::Undefined => ViviswapVerificationStep::General,
            KycStep::General => ViviswapVerificationStep::Personal,
            KycStep::Personal => ViviswapVerificationStep::Residence,
            KycStep::Residence => ViviswapVerificationStep::Identity,
            KycStep::Identity => ViviswapVerificationStep::Amla,
            KycStep::Amla => ViviswapVerificationStep::Documents,
            KycStep::Document => ViviswapVerificationStep::Undefined,
            _ => ViviswapVerificationStep::Undefined,
        };

        // update state internally
        if let Some(repo) = &mut self.repo {
            repo.set_viviswap_kyc_state(
                &username,
                verification_status.clone(),
                monthly_limit_eur,
                next_verification_step,
            )?;
        } else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        Ok(ViviswapKycStatus {
            full_name: kyc_status.full_name,
            monthly_limit_eur,
            verified_step: kyc_status.verified_step.into(),
            submission_step: kyc_status.submission_step.into(),
            verification_status,
        })
    }

    /// Submit the previously entered partial kyc details for viviswap.
    ///
    /// # Errors
    ///
    /// Returns a vector of [`crate::Error`] if any of the following conditions are met:
    ///
    /// - Repository initialization error.
    /// - Viviswap missing user error.
    /// - Viviswap invalid state error.
    /// - Viviswap missing field error.
    /// - Viviswap API error.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the submission is successful.
    pub async fn submit_kyc_partially_status_for_viviswap(&mut self) -> Result<()> {
        info!("Submitting partial KYC status for viviswap");

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        // load user entity
        let mut user = self.get_user().await?;

        // ensure that the repository exist (cannot borrow as mutable here since we also borrow self as mutable in between)
        if self.repo.is_none() {
            return Err(crate::Error::UserRepoNotInitialized);
        }

        // check if user has already a viviswap state available
        let Some(viviswap_state) = &mut user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        };

        // check if user is in valid state
        if viviswap_state.verification_status != ViviswapVerificationStatus::Unverified {
            return Err(crate::Error::Viviswap(ViviswapError::InvalidState));
        };

        // check if all required fields are available
        let mut missing_field_errors = Vec::new();
        if viviswap_state.partial_kyc_details_input.is_individual.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "is_individual".to_string(),
            }));
        }
        if viviswap_state.partial_kyc_details_input.is_pep.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "is_pep".to_string(),
            }));
        }
        if viviswap_state.partial_kyc_details_input.is_us_citizen.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "is_us_citizen".to_string(),
            }));
        }
        if viviswap_state
            .partial_kyc_details_input
            .is_regulatory_disclosure
            .is_none()
        {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "is_regulatory_disclosure".to_string(),
            }));
        }
        if viviswap_state.partial_kyc_details_input.country_of_residence.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "country_of_residence".to_string(),
            }));
        }
        if viviswap_state.partial_kyc_details_input.nationality.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "nationality".to_string(),
            }));
        }
        if viviswap_state.partial_kyc_details_input.full_name.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "full_name".to_string(),
            }));
        }
        if viviswap_state.partial_kyc_details_input.date_of_birth.is_none() {
            missing_field_errors.push(crate::Error::Viviswap(ViviswapError::MissingField {
                field: "date_of_birth".to_string(),
            }));
        }

        // check if a missing field error exists and return if so
        if !missing_field_errors.is_empty() {
            return Err(crate::Error::Viviswap(ViviswapError::Aggregate(missing_field_errors)));
        }

        // destructure the contents of the struct
        let ViviswapPartiallyKycDetails {
            is_individual: Some(is_individual),
            is_pep: Some(is_pep),
            is_us_citizen: Some(is_us_citizen),
            is_regulatory_disclosure: Some(is_regulatory_disclosure),
            country_of_residence: Some(country_of_residence),
            nationality: Some(nationality),
            full_name: Some(full_name),
            date_of_birth: Some(date_of_birth),
        } = &viviswap_state.partial_kyc_details_input
        else {
            // SAFETY: we have already checked that all are not None, and if any are we have returned an error
            unreachable!()
        };

        let access_token = self
            .access_token
            .as_ref()
            .ok_or_else(|| crate::error::Error::MissingAccessToken)?;

        // submit viviswap general kyc details
        if viviswap_state.next_verification_step == ViviswapVerificationStep::General {
            set_viviswap_kyc_general_details(
                config,
                access_token,
                *is_individual,
                *is_pep,
                *is_us_citizen,
                *is_regulatory_disclosure,
                country_of_residence,
                nationality,
            )
            .await?;
        }

        // submit viviswap personal kyc details
        set_viviswap_kyc_personal_details(config, access_token, full_name, date_of_birth).await?;

        // get new verification status of viviswap user
        self.get_kyc_details_for_viviswap().await?;

        // update users state
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        user.viviswap_state = Some(viviswap_state.clone());
        repo.update(&user)?;

        Ok(())
    }

    /// Update the kyc details for viviswap to be submitted
    ///
    /// # Arguments
    ///
    /// * `is_individual` - Whether the user is an individual.
    /// * `is_pep` - Whether the user is a politically exposed person.
    /// * `is_us_citizen` - Whether the user is a US citizen.
    /// * `is_regulatory_disclosure` - Whether the user has accepted the regulatory disclosure.
    /// * `country_of_residence` - The country of residence of the user.
    /// * `nationality` - The nationality of the user.
    /// * `full_name` - The full name of the user.
    /// * `date_of_birth` - The date of birth of the user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the partially updated KYC details or a vector of errors.
    ///
    /// # Errors
    ///
    /// Returns a vector of errors if any validation errors occur during the update process.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_kyc_partially_status_for_viviswap(
        &mut self,
        is_individual: Option<bool>,
        is_pep: Option<bool>,
        is_us_citizen: Option<bool>,
        is_regulatory_disclosure: Option<bool>,
        country_of_residence: Option<String>,
        nationality: Option<String>,
        full_name: Option<String>,
        date_of_birth: Option<String>,
    ) -> Result<ViviswapPartiallyKycDetails> {
        info!("Updating partial KYC status of user in viviswap");

        // load user entity
        let mut user = self.get_user().await?;

        // load repository
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        // check if user has already a viviswap state available
        let Some(viviswap_state) = &mut user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        };

        // check if user is in valid state
        if viviswap_state.verification_status != ViviswapVerificationStatus::Unverified {
            return Err(crate::Error::Viviswap(ViviswapError::InvalidState));
        };

        // run validators on each field and set if valid
        let mut field_validation_errors = Vec::new();
        let mut partial_kyc_details_input = viviswap_state.partial_kyc_details_input.clone();

        if is_individual.is_some() {
            partial_kyc_details_input.is_individual = is_individual;
        }
        if is_pep.is_some() {
            if is_pep != Some(false) {
                field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                    String::from("is_pep"),
                    String::from("You are not allowed to be a pep!"),
                )));
            } else {
                partial_kyc_details_input.is_pep = is_pep;
            }
        }
        if is_us_citizen.is_some() {
            if is_us_citizen != Some(false) {
                field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                    String::from("is_us_citizen"),
                    String::from("You are not allowed to be a us citizen!"),
                )));
            } else {
                partial_kyc_details_input.is_us_citizen = is_us_citizen;
            }
        }
        if is_regulatory_disclosure.is_some() {
            if is_regulatory_disclosure != Some(true) {
                field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                    String::from("is_regulatory_disclosure"),
                    String::from("You must accept the regulatory disclosure!"),
                )));
            } else {
                partial_kyc_details_input.is_regulatory_disclosure = is_regulatory_disclosure;
            }
        }
        if let Some(country_of_residence_val) = country_of_residence {
            if country_of_residence_val.len() != 2
                || !vec![
                    "AW", "AO", "AI", "AX", "AD", "AR", "AM", "AS", "AQ", "TF", "AG", "AU", "AZ", "BI", "BJ", "BS",
                    "BA", "BL", "BY", "BZ", "BM", "BO", "BN", "BT", "BV", "BW", "CF", "CC", "CL", "CN", "CI", "CM",
                    "CD", "CK", "CO", "KM", "CV", "CR", "CU", "CW", "CX", "DJ", "DM", "DO", "EC", "ER", "EH", "ET",
                    "FJ", "FK", "FO", "FM", "GA", "GE", "GH", "GN", "GP", "GM", "GW", "GQ", "GD", "GL", "GT", "GF",
                    "GU", "GY", "HK", "HM", "HN", "IN", "IO", "IS", "IL", "JP", "KE", "KG", "KI", "KN", "KR", "XK",
                    "LA", "LR", "LC", "LS", "MO", "MF", "MC", "MD", "MG", "MV", "MX", "MH", "MK", "MN", "MP", "MS",
                    "MQ", "MU", "MW", "YT", "NA", "NC", "NE", "NF", "NU", "NP", "NR", "NZ", "PN", "PE", "PW", "PG",
                    "PR", "PY", "PF", "RE", "RU", "RW", "SG", "GS", "SJ", "SB", "SL", "SV", "SM", "PM", "RS", "ST",
                    "SR", "SZ", "SX", "SC", "TC", "TD", "TG", "TH", "TJ", "TK", "TM", "TL", "TO", "TV", "TW", "UA",
                    "UM", "UY", "UZ", "VA", "VC", "VE", "VG", "VI", "VN", "WF", "WS", "ZA", "ZM", "AT", "BE", "BG",
                    "BR", "CA", "HR", "CY", "CZ", "DK", "DE", "EE", "FI", "FR", "GR", "GG", "IE", "IM", "IT", "JE",
                    "KZ", "LV", "LI", "LT", "LU", "ME", "MT", "NL", "NO", "PL", "PT", "RO", "SK", "SI", "ES", "SE",
                    "CH", "GB", "HU",
                ]
                .contains(&country_of_residence_val.to_uppercase().as_str())
            {
                field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                    String::from("country_of_residence"),
                    String::from("The country of residence is not valid or your country is not allowed as residence!"),
                )));
            } else {
                partial_kyc_details_input.country_of_residence = Some(country_of_residence_val);
            }
        }
        if let Some(nationality_val) = nationality {
            if nationality_val.len() != 2
                || !vec![
                    "BD", "DZ", "EG", "ID", "IQ", "KW", "LB", "LY", "LK", "MR", "MY", "NG", "OM", "PS", "QA", "SD",
                    "SA", "TN", "AW", "AO", "AI", "AX", "AD", "AR", "AM", "AS", "AQ", "TF", "AG", "AU", "AZ", "BI",
                    "BJ", "BS", "BA", "BL", "BY", "BZ", "BM", "BO", "BN", "BT", "BV", "BW", "CF", "CC", "CL", "CN",
                    "CI", "CM", "CD", "CK", "CO", "KM", "CV", "CR", "CU", "CW", "CX", "DJ", "DM", "DO", "EC", "ER",
                    "EH", "ET", "FJ", "FK", "FO", "FM", "GA", "GE", "GH", "GN", "GP", "GM", "GW", "GQ", "GD", "GL",
                    "GT", "GF", "GU", "GY", "HK", "HM", "HN", "IN", "IO", "IS", "IL", "JP", "KE", "KG", "KI", "KN",
                    "KR", "XK", "LA", "LR", "LC", "LS", "MO", "MF", "MC", "MD", "MG", "MV", "MX", "MH", "MK", "MN",
                    "MP", "MS", "MQ", "MU", "MW", "YT", "NA", "NC", "NE", "NF", "NU", "NP", "NR", "NZ", "PN", "PE",
                    "PW", "PG", "PR", "PY", "PF", "RE", "RU", "RW", "SG", "GS", "SJ", "SB", "SL", "SV", "SM", "PM",
                    "RS", "ST", "SR", "SZ", "SX", "SC", "TC", "TD", "TG", "TH", "TJ", "TK", "TM", "TL", "TO", "TV",
                    "TW", "UA", "UM", "UY", "UZ", "VA", "VC", "VE", "VG", "VI", "VN", "WF", "WS", "ZA", "ZM", "AT",
                    "BE", "BG", "BR", "CA", "HR", "CY", "CZ", "DK", "DE", "EE", "FI", "FR", "GR", "GG", "IE", "IM",
                    "IT", "JE", "KZ", "LV", "LI", "LT", "LU", "ME", "MT", "NL", "NO", "PL", "PT", "RO", "SK", "SI",
                    "ES", "SE", "CH", "GB", "HU",
                ]
                .contains(&nationality_val.to_uppercase().as_str())
            {
                field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                    String::from("nationality"),
                    String::from("The nationality is not valid or your country is not allowed as nationality!"),
                )));
            } else {
                partial_kyc_details_input.nationality = Some(nationality_val);
            }
        }

        if let Some(full_name_val) = full_name {
            if full_name_val.len() < 2 || full_name_val.len() > 128 {
                field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                    String::from("full_name"),
                    String::from("The full name is not valid! Must be between 2 and 128 characters."),
                )));
            } else {
                partial_kyc_details_input.full_name = Some(full_name_val);
            }
        }

        if let Some(date_of_birth_val) = date_of_birth {
            let min_birth_date = Utc::now().date_naive() - chrono::Duration::days(18 * 365); // computes the minimum birth date allowed
            match NaiveDate::parse_from_str(date_of_birth_val.clone().as_str(), "%Y-%m-%d") {
                Ok(birth_date) => {
                    if birth_date <= min_birth_date {
                        partial_kyc_details_input.date_of_birth = Some(date_of_birth_val);
                    } else {
                        field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                            String::from("date_of_birth"),
                            String::from("The date of birth is not valid! Must be older than 18 years."),
                        )));
                    }
                }
                Err(_) => {
                    field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                        String::from("date_of_birth"),
                        String::from("The date of birth is not valid! Must have format YYYY-MM-DD."),
                    )));
                }
            }
        }

        // check if a missing field error exists and return if so
        if !field_validation_errors.is_empty() {
            return Err(crate::Error::Viviswap(ViviswapError::Aggregate(
                field_validation_errors,
            )));
        }

        viviswap_state.partial_kyc_details_input = partial_kyc_details_input.clone();
        repo.update(&user)?;

        Ok(partial_kyc_details_input)
    }

    /// Set KYC identity details
    ///
    /// # Arguments
    ///
    /// - `official_document` - The official document that verifies the person (eg. ID-Card, Passport, Drivers License …).
    /// - `personal_document` - A 30 second video document that verifies that this person is willing to verify at viviswap and that the person really is the one they claim to be.
    ///
    /// # Errors
    ///
    /// - [[`crate::Error::UserNotInitialized)`]] - If the user is not initialized.
    /// - [[`crate::Error::ViviswapApiError`]] - If there is an error in the viviswap API.
    pub async fn set_viviswap_kyc_identity_details(
        &self,
        official_document: IdentityOfficialDocumentData,
        personal_document: IdentityPersonalDocumentData,
    ) -> Result<()> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        backend::viviswap::set_viviswap_kyc_identity_details(
            config,
            access_token,
            official_document,
            personal_document,
        )
        .await?;
        Ok(())
    }

    /// Set KYC residence details
    ///
    /// # Arguments
    ///
    /// - `country_code`, `region`, `zip_code`, `city`, `address_line_1`, `address_line_2` - basic address data.
    /// - `is_public_entry` - Inidcates that a valid public entry of this clients address can be found.
    /// - `public_entry_reference` - if `is_public_entry` is `true`, then this must contain the resource link.
    /// - `has_no_official_document` - indicates if the client does not have any document verifying their address.
    /// - `document_residence_proof` - if `has_no_official_document` is `false`, then this must contain the document file
    ///    that verifies that this person is currently living at the address submitted.
    ///
    ///
    /// # Errors
    ///
    /// - [[`crate::Error::UserNotInitialized)`]] - If the user is not initialized.
    /// - [[`crate::Error::ViviswapValidation`]] - If the input values are not valid.
    /// - [[`crate::Error::ViviswapApiError`]] - If there is an error in the viviswap API.
    #[allow(clippy::too_many_arguments)]
    pub async fn set_viviswap_kyc_residence_details(
        &self,
        country_code: String,
        region: String,
        zip_code: String,
        city: String,
        address_line_1: String,
        address_line_2: String,
        is_public_entry: bool,
        public_entry_reference: Option<String>,
        has_no_official_document: bool,
        document_residence_proof: Option<File>,
    ) -> Result<()> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let mut field_validation_errors = Vec::new();
        // do some validation checks
        if is_public_entry && public_entry_reference.is_none() {
            field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                String::from("public_entry_reference"),
                String::from("if is_public_entry is true, public_entry_reference must not be _None_"),
            )));
        }

        if !has_no_official_document && document_residence_proof.is_none() {
            field_validation_errors.push(crate::Error::Viviswap(ViviswapError::Validation(
                String::from("document_residence_proof"),
                String::from("if has_no_official_document is false, document_residence_proof must not be _None_"),
            )));
        }

        // check if a missing field error exists and return if so
        if !field_validation_errors.is_empty() {
            return Err(crate::Error::Viviswap(ViviswapError::Aggregate(
                field_validation_errors,
            )));
        }

        let access_token = self
            .access_token
            .as_ref()
            .ok_or_else(|| crate::error::Error::MissingAccessToken)?;
        backend::viviswap::set_viviswap_kyc_residence_details(
            config,
            access_token,
            api_types::api::viviswap::kyc::SetResidenceDataRequest {
                country_code,
                region,
                zip_code,
                city,
                address_line_1,
                address_line_2,
                is_public_entry,
                public_entry_reference,
                has_no_official_document,
                document_residence_proof,
            },
        )
        .await?;
        Ok(())
    }

    /// Get the open AMLA KYC questions
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// - A list of the currently open AMLA questions.
    ///
    /// # Errors
    ///
    /// - [[`crate::Error::UserNotInitialized)`]] - If the user is not initialized.
    /// - [[`crate::Error::ViviswapApiError`]] - If there is an error in the viviswap API.
    pub async fn get_viviswap_kyc_amla_open_questions(&self) -> Result<Vec<KycAmlaQuestion>> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let amla_questions = backend::viviswap::get_viviswap_kyc_amla_open_questions(config, access_token)
            .await
            .map(|v| v.questions)?;
        Ok(amla_questions)
    }

    /// Set the answer to an open AMLA KYC question
    ///
    /// # Arguments
    ///
    /// - `question_id` - The ID of the question to set the answer to.
    /// - `answers` - a list of the selected available answers for the question.
    /// - `freetext_answer` - an optional free-text answer.
    ///
    /// # Errors
    ///
    /// - [[`crate::Error::UserNotInitialized)`]] - If the user is not initialized.
    /// - [[`crate::Error::ViviswapApiError`]] - If there is an error in the viviswap API.
    pub async fn set_viviswap_kyc_amla_answer(
        &self,
        question_id: String,
        answers: Vec<String>,
        freetext_answer: Option<String>,
    ) -> Result<()> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        backend::viviswap::set_viviswap_kyc_amla_answer(
            config,
            access_token,
            api_types::api::viviswap::kyc::AnswerData {
                question_id,
                answers,
                freetext_answer,
            },
        )
        .await?;
        Ok(())
    }

    /// Get the currently open/missing documents for KYC
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// - A list of the currently open documents.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::UserNotInitialized)`] - If the user is not initialized.
    /// - [`crate::Error::ViviswapApiError`] - If there is an error in the viviswap API.
    pub async fn get_viviswap_kyc_open_documents(&self) -> Result<Vec<KycOpenDocument>> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        Ok(backend::viviswap::get_viviswap_kyc_open_documents(config, access_token)
            .await
            .map(|v| v.documents)?)
    }

    /// Set / upload an open KYC document
    ///
    /// # Arguments
    ///
    /// - `document_id` - The ID of the document to upload.
    /// - `expiration_date` - the expiration date of this document.
    /// - `document_number` - the official document number.
    /// - `front_image` - the front image of the official document.
    /// - `back_image` - the back image of the official document.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::ViviswapApiError`] - If there is an error in the viviswap API.
    /// - [`crate::Error::UserNotInitialized)`] - If the user is not initialized.
    pub async fn set_viviswap_kyc_document(
        &self,
        document_id: String,
        expiration_date: String,
        document_number: String,
        front_image: Option<File>,
        back_image: Option<File>,
    ) -> Result<()> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        backend::viviswap::set_viviswap_kyc_document(
            config,
            access_token,
            api_types::api::viviswap::kyc::SetDocumentDataRequest {
                document_id,
                expiration_date,
                document_number,
                front_image,
                back_image,
            },
        )
        .await?;
        Ok(())
    }
}
