sea_query::sea_query_driver_postgres!();
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use sea_query::{Iden, OnConflict, PostgresQueryBuilder, Query, Values};
use sea_query_driver_postgres::bind_query;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;
use chrono::{Local, NaiveDateTime, Utc};
use near_primitives::types::AccountId;
use crate::anchor::types::{AppchainId, ValidatorSetProcessingStatus};
use crate::{DB_POOL, ValidatorSetInfo};

#[derive(Iden)]
#[iden(rename = "validator_set")]
enum ValidatorSetTable {
    Table,
    AppchainId,
    EraNumber,
    TotalStake,
    StartBlockHeight,
    StartTimestamp,
    StartTimestampDate,
    StakingHistoryIndex,
    UnprofitableValidatorIds,
    ValidTotalStake,
    ProcessingStatus,
    UpdateTime
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ValidatorSetStruct {
    pub appchain_id: String,
    pub era_number: BigDecimal,
    pub total_stake: BigDecimal,
    pub start_block_height: BigDecimal,
    pub start_timestamp: BigDecimal,
    pub start_timestamp_date: NaiveDateTime,
    pub staking_history_index: BigDecimal,
    pub unprofitable_validator_ids: String, // Vec<AccountId>
    pub valid_total_stake: BigDecimal,
    pub processing_status: Value, //ValidatorSetProcessingStatus,
    pub update_time: NaiveDateTime,
}

impl ValidatorSetStruct {

    pub fn from_validator_set_info(
        validator_set_info: ValidatorSetInfo,
        appchain_id: AppchainId,
    )->Self {
        dbg!(&validator_set_info);
        Self {
            appchain_id,
            era_number: validator_set_info.era_number.into(),
            total_stake: BigDecimal::from_str(validator_set_info.total_stake.to_string().as_str()).unwrap(),
            start_block_height: validator_set_info.start_block_height.into(),
            start_timestamp: validator_set_info.start_timestamp.into(),
            // todo correct by local
            start_timestamp_date: Utc::now().naive_utc(),
            staking_history_index: validator_set_info.staking_history_index.into(),
            unprofitable_validator_ids: validator_set_info.unprofitable_validator_ids.join(","),
            valid_total_stake: BigDecimal::from_str(validator_set_info.valid_total_stake.to_string().as_str()).expect("Failed to convert u128 to BigDecimal."),
            // todo verify
            processing_status: json!(validator_set_info.processing_status),
            // correct local date time
            update_time: Utc::now().naive_utc()
        }

    }

    pub fn build_save_sql(validator_set_list: &[ValidatorSetStruct]) -> (String, Values) {
        let mut query = Query::insert()
            .into_table(ValidatorSetTable::Table)
            .to_owned();
        query.columns(vec![
            ValidatorSetTable::AppchainId,
            ValidatorSetTable::EraNumber,
            ValidatorSetTable::TotalStake,
            ValidatorSetTable::StartBlockHeight,
            ValidatorSetTable::StartTimestamp,
            ValidatorSetTable::StartTimestampDate,
            ValidatorSetTable::StakingHistoryIndex,
            ValidatorSetTable::UnprofitableValidatorIds,
            ValidatorSetTable::ValidTotalStake,
            ValidatorSetTable::ProcessingStatus,
            ValidatorSetTable::UpdateTime,
        ]);

        for validator_set in validator_set_list {
            query.values(vec![
                validator_set.appchain_id.clone().into(),
                validator_set.era_number.clone().into(),
                validator_set.total_stake.clone().into(),
                validator_set.start_block_height.clone().into(),
                validator_set.start_timestamp.clone().into(),
                validator_set.start_timestamp_date.clone().into(),
                validator_set.staking_history_index.clone().into(),
                validator_set.unprofitable_validator_ids.clone().into(),
                validator_set.valid_total_stake.clone().into(),
                validator_set.processing_status.clone().into(),
                validator_set.update_time.clone().into()
            ]).expect("DB query data fail validator_set");
        }
        query.build(PostgresQueryBuilder)
    }

    pub async fn save(validator_set_list: &[ValidatorSetStruct]) -> Result<()> {
        if validator_set_list.is_empty() {
            return Ok(());
        }
        let (sql, values) = ValidatorSetStruct::build_save_sql(validator_set_list);
        let _row = bind_query(sqlx::query(&sql), &values)
            .fetch_all(DB_POOL.get().await)
            .await?;
        Ok(())
    }


}