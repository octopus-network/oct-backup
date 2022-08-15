pub mod anchor_backup;

use chrono::Utc;
use near_primitives::types::AccountId;
use serde_json::json;
use crate::{AnchorContract, AnchorView, ValidatorSetInfo, ValidatorSetStruct};
use crate::anchor::types::{get_appchain_id_from_contract_id, ValidatorSetProcessingStatus};



async fn backup_validators_info() {

}

async fn backup_delegators_info() {

}