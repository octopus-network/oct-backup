use near_primitives::types::AccountId;
use serde::{Deserialize, Serialize};
use crate::util::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidatorSetInfo {
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub era_number: u64,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub total_stake: u128,
    pub validator_list: Vec<AppchainValidator>,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub start_block_height: u64,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub start_timestamp: u64,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub staking_history_index: u64,
    pub unprofitable_validator_ids: Vec<AccountId>,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub valid_total_stake: u128,
    pub processing_status: ValidatorSetProcessingStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppchainValidator {
    pub validator_id: AccountId,
    pub validator_id_in_appchain: String,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub deposit_amount: u128,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub total_stake: u128,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub delegators_count: u64,
    pub can_be_delegated_to: bool,
    pub is_unbonding: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppchainDelegator {
    pub delegator_id: AccountId,
    pub validator_id: AccountId,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub delegation_amount: u128,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ValidatorSetProcessingStatus {
    CopyingFromLastEra {
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        copying_validator_index: u64,
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        copying_delegator_index: u64,
    },
    ApplyingStakingHistory {
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        applying_index: u64,
    },
    ReadyForDistributingReward,
    DistributingReward {
        appchain_message_nonce: u32,
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        distributing_validator_index: u64,
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        distributing_delegator_index: u64,
    },
    Completed,
    UnbondingValidator {
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        unbonding_validator_index: u64,
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        unbonding_delegator_index: u64,
    },
    AutoUnbondingValidator {
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        unbonding_validator_index: u64,
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        unbonding_delegator_index: u64,
    },
    CheckingForAutoUnbondingValidator {
        #[serde(default)]
        #[serde(with = "u64_dec_format")]
        unprofitable_validator_index: u64,
    },
}

pub type AppchainId = String;

pub fn get_appchain_id_from_contract_id(contract_id: &AccountId)-> AppchainId {
    let split: Vec<_> = contract_id.split(".").collect();
    split[0].clone().to_string()
}


#[derive(Serialize, Deserialize, Clone)]
pub struct RewardHistory {
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub era_number: u64,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub total_reward: u128,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub unwithdrawn_reward: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StakingHistory {
    pub staking_fact: StakingFact,
    #[serde(with = "u64_dec_format")]
    pub block_height: u64,
    #[serde(with = "u64_dec_format")]
    pub timestamp: u64,
    #[serde(with = "u64_dec_format")]
    pub index: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StakingFact {
    /// A new validator is registered in appchain anchor
    ValidatorRegistered {
        validator_id: AccountId,
        validator_id_in_appchain: String,
        #[serde(with = "u128_dec_format")]
        amount: u128,
        can_be_delegated_to: bool,
    },
    /// A validator increases his stake in appchain anchor
    StakeIncreased {
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A validator decreases his stake in appchain anchor
    StakeDecreased {
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A validator unbonded his stake in appchain anchor
    ValidatorUnbonded {
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// The flag of `can_be_delegated_to` is set to `true`
    ValidatorDelegationEnabled { validator_id: AccountId },
    /// The flag of `can_be_delegated_to` is set to `false`
    ValidatorDelegationDisabled { validator_id: AccountId },
    /// A new delegator is registered in appchain anchor
    DelegatorRegistered {
        delegator_id: AccountId,
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A delegator increases his delegation for a validator in appchain anchor
    DelegationIncreased {
        delegator_id: AccountId,
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A delegator decreases his delegation for a validator in appchain anchor
    DelegationDecreased {
        delegator_id: AccountId,
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A delegator unbonded his delegation for a validator in appchain anchor
    DelegatorUnbonded {
        delegator_id: AccountId,
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A validator is unbonded by contract automatically
    ValidatorAutoUnbonded {
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A delegator is unbonded by contract automatically
    DelegatorAutoUnbonded {
        delegator_id: AccountId,
        validator_id: AccountId,
        #[serde(with = "u128_dec_format")]
        amount: u128,
    },
    /// A validator's account id in appchain changed
    ValidatorIdInAppchainChanged {
        validator_id: AccountId,
        validator_id_in_appchain: String,
    },
}