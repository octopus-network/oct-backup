use std::str::FromStr;

use bigdecimal::BigDecimal;
use itertools::Itertools;
use tracing::{debug, instrument, warn};

use crate::{AccountId, AnchorContract, AnchorView, ValidatorSetInfo, ValidatorSetStruct};
use crate::anchor::types::{get_appchain_id_from_contract_id, RewardHistory};
use crate::db::delegator_infos::DelegatorInfoStruct;
use crate::db::staking_histories::StakingHistoryStruct;
use crate::db::validator_infos::ValidatorInfoStruct;
#[instrument(level = "debug")]
pub async fn backup_staking_histories(contract_id: AccountId,start_index: u64, quantity: Option<u64>)->anyhow::Result<Vec<StakingHistoryStruct>> {
    let anchor_contract =  AnchorContract{ contract_account_id:  contract_id.clone()};
    let staking_histories = anchor_contract.get_staking_histories(start_index, quantity).await;
    let appchain_id = get_appchain_id_from_contract_id(&anchor_contract.contract_account_id);
    let staking_history_struct_vec = staking_histories.iter().map(|e| StakingHistoryStruct::from_staking_histories(
        e.clone(), appchain_id.clone())).collect_vec();
    StakingHistoryStruct::save(&staking_history_struct_vec).await?;
    anyhow::Ok(staking_history_struct_vec)
}

#[instrument(level = "debug")]
pub async fn backup_anchor_validator_set(contract_id: AccountId, era_number: u64)->anyhow::Result<Option<(ValidatorSetStruct, Vec<ValidatorInfoStruct>, Vec<DelegatorInfoStruct>)>>{
    let anchor_contract =  AnchorContract{ contract_account_id:  contract_id.clone()};
    let info_option = anchor_contract.get_validator_set_info_of(era_number).await;
    if info_option.is_none() {
        warn!("get_validator_set_info_of contract:{}, era:{}, and result is none, skip back up." , contract_id ,era_number);
        return anyhow::Ok(None)
    }
    let info = info_option.unwrap();
    let (validator_struct,
        validator_info_struct_vec,
        delegator_info_struct_vec) =
        get_backup_data_by_info(&anchor_contract, era_number, info).await;

    debug!("get validator_struct data: {:?}", validator_struct);
    debug!("get validator_info_struct_vec data: {:?}", validator_info_struct_vec);
    debug!("get delegator_info_struct_vec data: {:?}", delegator_info_struct_vec);

    ValidatorSetStruct::save(&vec![validator_struct.clone()]).await?;
    ValidatorInfoStruct::save(&validator_info_struct_vec).await?;
    DelegatorInfoStruct::save(&delegator_info_struct_vec).await?;

    anyhow::Ok(Some((validator_struct, validator_info_struct_vec, delegator_info_struct_vec)))
}

pub async fn get_backup_data_by_info(
    contract: &AnchorContract,
    era_number: u64,
    info: ValidatorSetInfo
)->(ValidatorSetStruct, Vec<ValidatorInfoStruct>, Vec<DelegatorInfoStruct>) {
    let appchain_id = get_appchain_id_from_contract_id(&contract.contract_account_id);
    let validator_set_struct =
        ValidatorSetStruct::from_validator_set_info(
            info.clone(),
            appchain_id.clone(),
        );

    let mut validator_info_struct_vec = vec![];
    let mut delegator_info_struct_vec = vec![];

    for appchain_validator in &info.validator_list {
        let history = get_validator_reward_of_one_era(&contract, era_number, appchain_validator.validator_id.clone()).await;
        let e = era_number;

        validator_info_struct_vec.push(ValidatorInfoStruct::from_appchain_validator(
            appchain_validator.clone(),
            appchain_id.clone(),
            e.into(),
            BigDecimal::from_str(history.total_reward.to_string().as_str()).unwrap(),
            BigDecimal::from_str(history.unwithdrawn_reward.to_string().as_str()).unwrap()
        ));

        let delegator_vec = contract.get_delegators_of_validator_in_era(era_number, appchain_validator.validator_id.clone()).await;
        for appchain_delegator in &delegator_vec {
            let reward_history= get_delegator_reward_of_one_era(
                &contract,
                era_number,
                appchain_delegator.delegator_id.clone(),
                appchain_delegator.validator_id.clone()
            ).await;
            delegator_info_struct_vec.push(
                DelegatorInfoStruct::from_appchain_delegator(
                    appchain_delegator.clone(),
                    appchain_id.clone(),
                    era_number.into(),
                    BigDecimal::from_str(reward_history.total_reward.to_string().as_str()).expect("Failed to convert u128 to BigDecimal."),
                    BigDecimal::from_str(reward_history.unwithdrawn_reward.to_string().as_str()).expect("Failed to convert u128 to BigDecimal.")
                )
            )
        }
    }

    (validator_set_struct, validator_info_struct_vec, delegator_info_struct_vec)
}


async fn get_validator_reward_of_one_era(
    anchor_contract: &AnchorContract,
    era: u64,
    validator_id: AccountId,
) -> RewardHistory {
    let reward_history_vec = anchor_contract.get_validator_rewards_of(era, era, validator_id.clone()).await;
    assert!(reward_history_vec.len()<=1, "Failed to get_validator_reward_of_one_era, len of reward_history should be 1. era is {}, validator_id is {}, contract_id is {}.", era, validator_id, anchor_contract.contract_account_id);
    if reward_history_vec.len()==0 {
        RewardHistory {
            era_number: era,
            total_reward: 0,
            unwithdrawn_reward: 0
        }
    } else {
        reward_history_vec[0].clone()
    }
}

async fn get_delegator_reward_of_one_era(
    anchor_contract: &AnchorContract,
    era: u64,
    delegator_id: AccountId,
    validator_id: AccountId,
) -> RewardHistory {
    let reward_history_vec = anchor_contract.get_delegator_rewards_of(era, era, delegator_id, validator_id).await;
    assert_eq!(reward_history_vec.len(), 1, "Failed to get_delegator_rewards_of_one_era, len of reward_history should be 1.");
    assert_eq!(reward_history_vec[0].era_number, era, "Failed to get_delegator_rewards_of_one_era, era_number of get_delegator_rewards_of should be equal to era.");
    reward_history_vec[0].clone()
}