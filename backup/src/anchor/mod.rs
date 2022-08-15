use chrono::Utc;
use serde_json::json;
use crate::{AccountId, AnchorContract, AnchorView, ValidatorSetInfo};
use crate::anchor::types::{get_appchain_id_from_contract_id, ValidatorSetProcessingStatus};
use crate::db::validator_set::ValidatorSetStruct;
use crate::global::ANCHOR_BACKUP;

pub mod contract_trait;
pub mod types;