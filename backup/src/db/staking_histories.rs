use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sea_query::{Iden, PostgresQueryBuilder, Query, Values};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use sea_query_driver_postgres::bind_query;

use crate::anchor::types::StakingHistory;
use crate::global::DB_POOL;
use crate::util::{naive_date_time_from_nanos_time, naive_date_time_now};

sea_query::sea_query_driver_postgres!();
#[allow(unused)]
#[derive(Iden)]
#[iden(rename = "staking_histories")]
enum StakingHistoriesTable {
    Table,
    AppchainId,
    StakingFact,
    BlockHeight,
    Timestamp,
    Index,
    TimestampDate,
    UpdateDate,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct StakingHistoryStruct {
    appchain_id: String,
    staking_fact: Value,
    block_height: BigDecimal,
    timestamp: BigDecimal,
    index: BigDecimal,
    timestamp_date: NaiveDateTime,
    update_date: NaiveDateTime,
}



impl StakingHistoryStruct {
    pub fn from_staking_histories(
        staking_history: StakingHistory,
        appchain_id: String
    ) -> Self {
        Self {
            appchain_id,
            staking_fact: json!(staking_history.staking_fact),
            block_height: staking_history.block_height.into(),
            timestamp: staking_history.timestamp.into(),
            index: staking_history.index.into(),
            timestamp_date: naive_date_time_from_nanos_time(staking_history.timestamp),
            update_date: naive_date_time_now(),
        }
    }


    pub fn build_save_sql(staking_histories: &[StakingHistoryStruct]) -> (String, Values) {
        let mut query = Query::insert()
            .into_table(StakingHistoriesTable::Table)
            .to_owned();
        query.columns(vec![
            StakingHistoriesTable::AppchainId,
            StakingHistoriesTable::StakingFact,
            StakingHistoriesTable::BlockHeight,
            StakingHistoriesTable::Timestamp,
            StakingHistoriesTable::Index,
            StakingHistoriesTable::TimestampDate,
            StakingHistoriesTable::UpdateDate,
        ]);

        for staking_history in staking_histories {
            query.values(vec![
                staking_history.appchain_id.clone().into(),
                staking_history.staking_fact.clone().into(),
                staking_history.block_height.clone().into(),
                staking_history.timestamp.clone().into(),
                staking_history.index.clone().into(),
                staking_history.timestamp_date.clone().into(),
                staking_history.update_date.clone().into(),
            ]).expect("DB staking_histories query data fail ");
        }
        query.build(PostgresQueryBuilder)
    }

    pub async fn save(staking_histories: &[StakingHistoryStruct]) -> anyhow::Result<()> {
        if staking_histories.is_empty() {
            println!("staking_histories is empty");
            return Ok(());
        }
        let (sql, values) = StakingHistoryStruct::build_save_sql(staking_histories);
        let _row = bind_query(sqlx::query(&sql), &values)
            .fetch_all(DB_POOL.get().await)
            .await?;
        Ok(())
    }
}