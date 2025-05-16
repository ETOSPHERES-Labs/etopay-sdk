use sdk::types::transactions::GasCostEstimation;
use sdk::types::transactions::PurchaseDetails;

#[derive(Debug, serde::Serialize)]
pub struct PurchaseDetailsEntity {
    /// The address where the amount should be paid.
    pub system_address: String,
    /// The amount to be paid.
    pub amount: f64,
    /// The status of transaction
    pub status: String,
    /// Any reasons attached to the status
    pub invalid_reasons: Vec<String>,
    /// Unique key representing a network that the transaction is sent in
    pub network_key: String,
}

impl TryFrom<PurchaseDetails> for PurchaseDetailsEntity {
    type Error = sdk::Error;
    fn try_from(val: PurchaseDetails) -> Result<Self, Self::Error> {
        let (status, status_reasons) = match val.status {
            sdk::types::ApiTxStatus::Pending => ("Pending", Vec::new()),
            sdk::types::ApiTxStatus::WaitingForVerification(vec) => ("WaitingForVerification", vec),
            sdk::types::ApiTxStatus::Valid => ("Valid", Vec::new()),
            sdk::types::ApiTxStatus::Invalid(vec) => ("Invalid", vec),
            sdk::types::ApiTxStatus::ProcessingIncoming => ("ProcessingIncoming", Vec::new()),
            sdk::types::ApiTxStatus::ProcessingOutgoing => ("ProcessingOutgoing", Vec::new()),
            sdk::types::ApiTxStatus::Completed => ("Completed", Vec::new()),
            sdk::types::ApiTxStatus::Failed => ("Failed", Vec::new()),
        };

        Ok(PurchaseDetailsEntity {
            system_address: val.system_address,
            amount: f64::try_from(val.amount)?,
            status: status.to_string(),
            network_key: val.network.key,
            invalid_reasons: status_reasons,
        })
    }
}

/// Gas estimation (EIP-1559)
#[derive(Debug, serde::Serialize)]
pub struct GasCostEstimationEntity {
    /// The maximum fee the sender is willing to pay per unit of gas.
    pub max_fee_per_gas: u128,
    /// The maximum tip the sender is willing to pay to miners (in EIP-1559).
    pub max_priority_fee_per_gas: u128,
    /// The maximum amount of gas that the transaction can consume.
    pub gas_limit: u64,
}

impl TryFrom<GasCostEstimation> for GasCostEstimationEntity {
    type Error = sdk::Error;
    fn try_from(val: GasCostEstimation) -> Result<Self, Self::Error> {
        Ok(GasCostEstimationEntity {
            max_fee_per_gas: val.max_fee_per_gas,
            max_priority_fee_per_gas: val.max_priority_fee_per_gas,
            gas_limit: val.gas_limit,
        })
    }
}
