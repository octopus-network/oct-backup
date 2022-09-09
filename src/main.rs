sea_query::sea_query_driver_postgres!();

use crate::anchor::contract_trait::{AnchorContract, AnchorView};
use near_primitives::types::{AccountId};
use crate::anchor::types::ValidatorSetInfo;
use crate::db::validator_set::ValidatorSetStruct;
use crate::global::{CMD_ARG, SYSTEM_ENV};
use crate::near::types::view;
use tracing_subscriber::{EnvFilter, filter};
use tracing_subscriber::layer::SubscriberExt;
use crate::cmd::{BackupType, Cmd, LogLevel};
use tracing::{debug, info, Level};
use tracing_subscriber::util::SubscriberInitExt;
use crate::backup::anchor_backup::{backup_anchor_validator_set, backup_staking_histories};
use itertools::Itertools;

mod anchor;
mod util;
mod global;
mod near;
mod db;
mod backup;
mod cmd;

#[tokio::main]
async fn main()-> anyhow::Result<()> {

    init_log().await;

    let cmd = CMD_ARG.get().await;
    match cmd.sub_cmd {
        BackupType::ValidatorSet { era, quantity , .. } => {
            debug!(era, quantity);
            for i in 0..quantity {
                backup_anchor_validator_set(
                    cmd.contract.clone().parse()?,
                    era+i).await?;
                info!("Finish back up validator set, era_number={}, quantity={}", era, quantity);
            }

        }
        BackupType::StakingHistory { start_index, quantity } => {
            let staking_histories = backup_staking_histories(
                cmd.contract.clone().parse()?,
                start_index,
                Some(quantity)).await?;
            info!("Finish back up staking histories of indexes: {}", staking_histories.iter().map(|e|e.index.clone()).join(","));
        }
    };
    Ok(())
}


async fn init_log() {
    let cmd: &Cmd = CMD_ARG.get().await;

    if cmd.log_level.is_some() {
        let level = match cmd.log_level.as_ref().unwrap() {
            LogLevel::TRACE => {Level::TRACE}
            LogLevel::DEBUG => {Level::DEBUG}
            LogLevel::INFO => {Level::INFO}
            LogLevel::WARN => {Level::WARN}
            LogLevel::ERROR => {Level::ERROR}
        };
        let filter = filter::Targets::new()
            .with_target("oct_backup", level);
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(filter).init();
        debug!("use cmd log level: {}",level);
    } else {
        let env = SYSTEM_ENV.get().await;
        if  env.rust_log.is_none() {

            let filter = filter::Targets::new()
                // Enable the `INFO` level for anything in `my_crate`
                .with_target("oct_backup", Level::INFO);
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer())
                .with(filter).init();
            debug!("use default log level: {}", Level::INFO);
        } else {
            let filter = EnvFilter::from_default_env();
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer()).with(filter).init();
            debug!("use env rust_log level: {}", env.rust_log.as_ref().unwrap());
        }
    }
}