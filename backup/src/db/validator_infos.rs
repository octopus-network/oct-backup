sea_query::sea_query_driver_postgres!();
use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDateTime, Utc};
use sea_query::{Iden, PostgresQueryBuilder, Query, Values};
use crate::anchor::types::{AppchainId, AppchainValidator};
use crate::{DB_POOL, ValidatorSetStruct};
use serde::{Deserialize, Serialize};
use sea_query_driver_postgres::bind_query;

#[derive(Iden)]
#[iden(rename = "validator_infos")]
enum ValidatorInfosTable {
    Table,
    AppchainId,
    EraNumber,
    ValidatorId,
    ValidatorIdInAppchain,
    DepositAmount,
    CanBeDelegatedTo,
    IsUnbonding,
    TotalReward,
    UnwithdrawnReward,
    UpdateDate
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ValidatorInfoStruct {
    pub appchain_id: String,
    pub era_number: BigDecimal,
    pub validator_id: String,
    pub validator_id_in_appchain: String,
    pub deposit_amount: BigDecimal,
    pub can_be_delegated_to: bool,
    pub is_unbonding: bool,
    pub total_reward: BigDecimal,
    pub unwithdrawn_reward: BigDecimal,
    pub update_date: NaiveDateTime
}

impl ValidatorInfoStruct {
    pub fn from_appchain_validator(
        appchain_validator: AppchainValidator,
        appchain_id: AppchainId,
        era_number: BigDecimal,
        total_reward: BigDecimal,
        unwithdrawn_reward: BigDecimal
    ) -> Self {
        Self {
            appchain_id,
            era_number,
            validator_id: appchain_validator.validator_id.to_string(),
            validator_id_in_appchain: appchain_validator.validator_id_in_appchain.to_string(),
            deposit_amount: BigDecimal::from_str(appchain_validator.deposit_amount.to_string().as_str()).expect("Failed to convert u128 to BigDecimal."),
            can_be_delegated_to: appchain_validator.can_be_delegated_to,
            is_unbonding: appchain_validator.is_unbonding,
            total_reward,
            // todo wait for add field
            update_date: Utc::now().naive_utc(),
            unwithdrawn_reward
        }

    }


    pub fn build_save_sql(validator_info_list: &[ValidatorInfoStruct]) -> (String, Values) {
        let mut query = Query::insert()
            .into_table(ValidatorInfosTable::Table)
            .to_owned();
        query.columns(vec![
            ValidatorInfosTable::AppchainId,
            ValidatorInfosTable::EraNumber,
            ValidatorInfosTable::ValidatorId,
            ValidatorInfosTable::ValidatorIdInAppchain,
            ValidatorInfosTable::DepositAmount,
            ValidatorInfosTable::CanBeDelegatedTo,
            ValidatorInfosTable::IsUnbonding,
            ValidatorInfosTable::TotalReward,
            ValidatorInfosTable::UnwithdrawnReward,
            ValidatorInfosTable::UpdateDate
        ]);

        for validator_info in validator_info_list {
            query.values(vec![
                validator_info.appchain_id.clone().into(),
                validator_info.era_number.clone().into(),
                validator_info.validator_id.clone().into(),
                validator_info.validator_id_in_appchain.clone().into(),
                validator_info.deposit_amount.clone().into(),
                validator_info.can_be_delegated_to.clone().into(),
                validator_info.is_unbonding.clone().into(),
                validator_info.total_reward.clone().into(),
                validator_info.unwithdrawn_reward.clone().into(),
                validator_info.update_date.clone().into()
            ]).expect("DB query data fail validator_set");

        }
        query.build(PostgresQueryBuilder)
    }

    pub async fn save(validator_info_list: &[ValidatorInfoStruct]) -> anyhow::Result<()> {
        if validator_info_list.is_empty() {
            println!("validator_info_list is empty");
            return Ok(());
        }
        let (sql, values) = ValidatorInfoStruct::build_save_sql(validator_info_list);
        let _row = bind_query(sqlx::query(&sql), &values)
            .fetch_all(DB_POOL.get().await)
            .await?;
        Ok(())
    }

}