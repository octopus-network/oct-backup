sea_query::sea_query_driver_postgres!();
use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDateTime, Utc};
use sea_query::{Iden, PostgresQueryBuilder, Query, Values};
use sqlx::types::Decimal;
use crate::{AccountId, DB_POOL};
use crate::anchor::types::{AppchainDelegator, AppchainId};
use crate::db::validator_infos::ValidatorInfoStruct;
use serde::{Deserialize, Serialize};
use sea_query_driver_postgres::bind_query;

#[allow(unused)]
#[derive(Iden)]
#[iden(rename = "delegator_infos")]
enum DelegatorInfosTable {
    Table,
    AppchainId,
    EraNumber,
    ValidatorId,
    DelegatorId,
    DepositAmount,
    TotalReward,
    UnwithdrawnReward,
    UpdateDate
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct DelegatorInfoStruct {
    appchain_id: String,
    era_number: BigDecimal,
    validator_id: String,
    delegator_id: String,
    deposit_amount: BigDecimal,
    total_reward: BigDecimal,
    unwithdrawn_reward: BigDecimal,
    update_date: NaiveDateTime
}

impl DelegatorInfoStruct {
    pub fn from_appchain_delegator(
        appchain_delegator: AppchainDelegator,
        appchain_id: AppchainId,
        era_number: BigDecimal,
        total_reward: BigDecimal,
        unwithdrawn_reward: BigDecimal
    )->Self {
        Self {
            appchain_id,
            era_number,
            validator_id: appchain_delegator.validator_id.to_string(),
            delegator_id: appchain_delegator.delegator_id.to_string(),
            deposit_amount: BigDecimal::from_str(appchain_delegator.delegation_amount.to_string().as_str()).unwrap(),
            // todo wait for add field
            total_reward,
            unwithdrawn_reward,
            // todo wait for add field
            update_date: Utc::now().naive_utc()
        }

    }

    pub fn build_save_sql(delegator_info_list: &[DelegatorInfoStruct]) -> (String, Values) {
        let mut query = Query::insert()
            .into_table(DelegatorInfosTable::Table)
            .to_owned();
        query.columns(vec![
            DelegatorInfosTable::AppchainId,
            DelegatorInfosTable::EraNumber,
            DelegatorInfosTable::ValidatorId,
            DelegatorInfosTable::DelegatorId,
            DelegatorInfosTable::DepositAmount,
            DelegatorInfosTable::UpdateDate
        ]);

        for delegator_info in delegator_info_list {
            query.values(vec![
               delegator_info.appchain_id.clone().into(),
                delegator_info.era_number.clone().into(),
                delegator_info.validator_id.clone().into(),
                delegator_info.delegator_id.clone().into(),
                delegator_info.deposit_amount.clone().into(),
                delegator_info.update_date.clone().into()
            ]).expect("DB query data fail validator_set");

        }
        query.build(PostgresQueryBuilder)
    }

    pub async fn save(delegator_info_list: &[DelegatorInfoStruct]) -> anyhow::Result<()> {
        if delegator_info_list.is_empty() {
            return Ok(());
        }
        let (sql, values) = DelegatorInfoStruct::build_save_sql(delegator_info_list);
        let _row = bind_query(sqlx::query(&sql), &values)
            .fetch_all(DB_POOL.get().await)
            .await?;
        Ok(())
    }


}