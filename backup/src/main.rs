sea_query::sea_query_driver_postgres!();

use std::env;
use crate::anchor::contract_trait::{AnchorContract, AnchorView};
use near_primitives::types::{AccountId};
use crate::anchor::types::ValidatorSetInfo;
use crate::db::validator_set::ValidatorSetStruct;
use crate::global::{CMD_ARG, DB_POOL, get_near_env, SYSTEM_ENV};
use crate::near::types::view;
use anyhow::Result;
use bigdecimal::{BigDecimal, One};
use chrono::Utc;
use sea_query_driver_postgres::bind_query;
use sea_query::{OnConflict, PostgresQueryBuilder, Query, Values};
use sqlx::PgPool;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use crate::db::validator_infos::ValidatorInfoStruct;
use crate::cmd::{BackupType, Cmd, NearEnv};
use structopt::StructOpt;
use crate::backup::anchor_backup::{backup_anchor_validator_set, backup_staking_histories};
use crate::global::ProjectConfig;


mod anchor;
mod util;
mod global;
mod near;
mod db;
mod backup;
mod cmd;


fn tmp() {
    // let opt: Cmd = Cmd::from_args();

    // dbg!(&opt.contract);
    // match opt.sub_cmd {
    //     BackupType::ValidatorSet { era, quantity, skip } => {
    //         dbg!(&skip);
    //         for i in 0..quantity {
    //             println!("{}", i+era);
    //
    //         }
    //         backup_anchor_validator_set(opt.contract.parse().unwrap(),1).await?;
    //     }
    //
    //     BackupType::StakingHistory { start_index, quantity } => {}
    // };


    // dbg!(&opt);

    // println!("{:?}", opt);
    // println!("just {:?}", env::var("JUST_NOTHING"));
    // for (n,v) in env::vars() {
    //     println!("{}: {}", n,v);
    // }
    // let env = PROJECT_CONFIG.get().await;
    // println!("{:?}", env.just_nothing);
    // println!("mainnet bool {:?}", env.mainnet_bool);


    //
    // dbg!(pool);

    // let result = test_insert().await;

    // backup_staking_histories("barnacle0918.registry.test_oct.testnet".parse().unwrap(),0, None).await?;

    //
    // let contract = AnchorContract {
    //     contract_account_id: "barnacle0918.registry.test_oct.testnet".parse().unwrap()
    // };
    // let result = contract.get_staking_histories(0, Some(5)).await;
    // dbg!(result);

    // let result= backup_anchor_validator_set("debionetwork.registry.test_oct.testnet".parse().unwrap(),1).await;
    // dbg!(&result);
    // let result = test_validator_info_insert().await;
    // dbg!(result);


    // dbg!(BigDecimal::from_str("100000000000000000000").unwrap());
    // dbg!(BigDecimal::from_u128(100000000000000000000).unwrap());
}

#[tokio::main]
async fn main()-> anyhow::Result<()> {

    let cmd = CMD_ARG.get().await;
    match cmd.sub_cmd {
        BackupType::ValidatorSet { era, quantity , .. } => {
            for i in 0..quantity {
                backup_anchor_validator_set(
                    cmd.contract.clone().parse()?,
                    era+i).await?;
            }
        }
        BackupType::StakingHistory { start_index, quantity } => {
            backup_staking_histories(
                cmd.contract.clone().parse()?,
                start_index,
                Some(quantity)).await?;
        }
    };
    Ok(())
}


async fn test_validator_info_insert() -> Result<()> {
    let datas = vec![ValidatorInfoStruct{
        appchain_id: "test".to_string(),
        era_number: Default::default(),
        validator_id: "".to_string(),
        validator_id_in_appchain: "".to_string(),
        deposit_amount: Default::default(),
        can_be_delegated_to: false,
        is_unbonding: false,
        total_reward: Default::default(),
        unwithdrawn_reward: Default::default(),
        update_date: Utc::now().naive_utc()
    }];

    let (sql, values) = ValidatorInfoStruct::build_save_sql(&datas);
    dbg!(&sql);

    let _row = bind_query(sqlx::query(&sql), &values)
        .fetch_all(DB_POOL.get().await)
        .await?;
    Ok(())

}

async fn test_insert()-> Result<()> {
    let data = vec![ValidatorSetStruct {
        appchain_id: "test1".to_string(),
        era_number: BigDecimal::one(),
        total_stake: BigDecimal::one(),
        start_block_height: Default::default(),
        start_timestamp: Default::default(),
        start_timestamp_date: Utc::now().naive_utc(),
        staking_history_index: Default::default(),
        unprofitable_validator_ids: "".to_string(),
        valid_total_stake: Default::default(),
        processing_status: Default::default(),
        update_time: Utc::now().naive_utc()
    }];
    let (sql, values) = ValidatorSetStruct::build_save_sql(&data);

    dbg!(&sql);

    let _row = bind_query(sqlx::query(&sql), &values)
        .fetch_all(DB_POOL.get().await)
        .await?;
    Ok(())
}
