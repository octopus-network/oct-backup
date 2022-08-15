use std::future::Future;
use crate::anchor::types::{AppchainDelegator, AppchainValidator, RewardHistory, StakingHistory, ValidatorSetInfo};
use anyhow::{anyhow, Result};
use near_primitives::types::{AccountId, FunctionArgs};
use serde_json::json;
use crate::view;
use std::option::Option;
use futures::FutureExt;
use async_trait::async_trait;

pub struct AnchorContract {
    pub contract_account_id: AccountId,
}

#[async_trait]
pub trait AnchorView {
    async fn get_validator_set_info_of(&self, era_number: u64) -> Option<ValidatorSetInfo>;

    async fn get_delegators_of_validator_in_era(
        &self,
        era_number: u64,
        validator_id: AccountId,
    ) -> Vec<AppchainDelegator>;

    async fn get_validator_rewards_of(
        &self,
        start_era: u64,
        end_era: u64,
        validator_id: AccountId,
    ) -> Vec<RewardHistory>;

    async fn get_delegator_rewards_of(
        &self,
        start_era: u64,
        end_era: u64,
        delegator_id: AccountId,
        validator_id: AccountId,
    ) -> Vec<RewardHistory>;

    async fn get_staking_histories(
        &self,
        start_index: u64,
        quantity: Option<u64>,
    ) -> Vec<StakingHistory>;
}

#[async_trait]
impl AnchorView for AnchorContract {
    async fn get_validator_set_info_of(&self, era_number: u64) -> Option<ValidatorSetInfo> {
        view(
            self.contract_account_id.clone(),
            "get_validator_set_info_of".to_string(),
            json!({"era_number": era_number.to_string()}),
        ).await.unwrap().json().expect("Failed to get_validator_set_info_of by deserialize error.")
    }

    async fn get_delegators_of_validator_in_era(
        &self,
        era_number: u64,
        validator_id: AccountId,
    ) -> Vec<AppchainDelegator> {
        view(
            self.contract_account_id.clone(),
            "get_delegators_of_validator_in_era".to_string(),
            json!({
                "era_number": era_number.to_string(),
                "validator_id": validator_id }),
        )
            .await
            .unwrap()
            .json()
            .expect("Failed to get_delegators_of_validator_in_era by deserialize error.")
    }


    async fn get_validator_rewards_of(&self, start_era: u64, end_era: u64, validator_id: AccountId) -> Vec<RewardHistory> {
        view(
            self.contract_account_id.clone(),
            "get_validator_rewards_of".to_string(),
            json!({
                "start_era": start_era.to_string(),
                "end_era": end_era.to_string(),
                "validator_id": validator_id.to_string()
            }),
        ).await.unwrap().json().expect("Failed to get_delegators_of_validator_in_era by deserialize error.")
    }

    async fn get_delegator_rewards_of(&self, start_era: u64, end_era: u64, delegator_id: AccountId, validator_id: AccountId) -> Vec<RewardHistory> {
        view(
            self.contract_account_id.clone(),
            "get_delegator_rewards_of".to_string(),
            json!({
                "start_era": start_era.to_string(),
                "end_era": end_era.to_string(),
                "validator_id": validator_id.to_string(),
                "delegator_id": delegator_id.to_string()
            }),
        ).await.unwrap().json().expect("Failed to get_delegators_of_validator_in_era by deserialize error.")
    }

    async fn get_staking_histories(
        &self,
        start_index: u64,
        quantity: Option<u64>,
    ) -> Vec<StakingHistory> {
        let quantity_unwrap = quantity.unwrap_or(1);
        view(
            self.contract_account_id.clone(),
            "get_staking_histories".to_string(),
            json!({
                "start_index": start_index.to_string(),
                "quantity": quantity_unwrap.to_string(),
            }),
        ).await.unwrap().json().expect("Failed to get_staking_histories by deserialize error.")

    }
}

pub async fn get_validator_set_info_of(contract_account_id: AccountId, era_number: u64) -> Option<ValidatorSetInfo> {
    // let a = rpc_call(contract_account_id,
    //          "get_validator_set_info_of".to_string(),
    //          FunctionArgs::from(json!({
    //             "era_number": era_number.to_string()}).to_string().into_bytes()))

    view(
        contract_account_id,
        "get_validator_set_info_of".to_string(),
        json!({"era_number": era_number.to_string()}),
    ).await.unwrap().json().unwrap()
}

