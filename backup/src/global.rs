sea_query::sea_query_driver_postgres!();

use std::env;
use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_MAINNET_RPC_URL, NEAR_TESTNET_RPC_URL};
use serde::{Deserialize, Serialize};
use async_once::AsyncOnce;
use sqlx::{PgPool, Pool, Postgres};
use sqlx::pool::PoolConnection;
use crate::cmd::{BackupType, Cmd, NearEnv};
use structopt::StructOpt;
use async_trait::async_trait;


pub static ANCHOR_BACKUP: &str = "anchor-back";

lazy_static! {
    pub static ref CMD_ARG: AsyncOnce<Cmd> = AsyncOnce::new(async {
        Cmd::from_args()
    });
    pub static ref SYSTEM_ENV: AsyncOnce<Env> = AsyncOnce::new(async {
        init_env_config().expect("Failed to init env config.")
    });
    pub static ref DEFAULT_NEAR_JSON_RPC_CLIENT: AsyncOnce<JsonRpcClient> =AsyncOnce::new( async {

        let cmd_arg = CMD_ARG.get().await;
        let rpc_url = if cmd_arg.node_url.is_some() {
            cmd_arg.node_url.clone().unwrap()
        } else {
            let env = SYSTEM_ENV.get().await;
            match env.near_env.as_ref().expect("Failed to ").as_str() {
                "mainnet" => env.near_cli_mainnet_rpc_server_url.clone().unwrap_or(NEAR_MAINNET_RPC_URL.to_string()),
                "testnet" => env.near_cli_testnet_rpc_server_url.clone().unwrap_or(NEAR_TESTNET_RPC_URL.to_string()),
                _ => panic!("Error near env")
            }
        };
        JsonRpcClient::connect(rpc_url)
    });
    pub static ref DB_POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new( async {
        // let database_url_from_cmd = CMD_ARG.get().await.database_url
        let database_url = SYSTEM_ENV.get().await.database_url.as_ref().expect("").as_str();
        PgPool::connect(database_url)
        .await
        .unwrap()
    });
}


pub struct Env {
    pub(crate) near_env: Option<String>,
    pub(crate) database_url: Option<String>,
    pub(crate) near_cli_testnet_rpc_server_url: Option<String>,
    pub(crate) near_cli_mainnet_rpc_server_url: Option<String>,
}

fn init_env_config() -> Result<Env> {
    Ok(Env {
        near_env: env::var("NEAR_ENV").ok(),
        database_url: env::var("DATABASE_URL").ok(),
        near_cli_testnet_rpc_server_url: env::var("NEAR_CLI_TESTNET_RPC_SERVER_URL").ok(),
        near_cli_mainnet_rpc_server_url: env::var("NEAR_CLI_MAINNET_RPC_SERVER_URL").ok(),
    })
}

pub async fn get_near_env() -> Option<NearEnv> {
    let cmd = CMD_ARG.get().await;
    if cmd.near_env.is_some() {
        return Some(cmd.near_env.clone().unwrap());
    }
    let env = SYSTEM_ENV.get().await;
    if env.near_env.is_some() {
        return Some(env.near_env.clone().unwrap().parse().unwrap());
    }
    None
}

async fn get_database_url() -> anyhow::Result<String> {
    let cmd = CMD_ARG.get().await;
    if cmd.database_url.is_some() {
        return anyhow::Ok(cmd.database_url.clone().unwrap());
    }
    let env = SYSTEM_ENV.get().await;
    if env.database_url.is_some() {
        return anyhow::Ok(env.database_url.clone().unwrap());
    }
    Err(anyhow!("Failed to get near env."))
}

// async fn get_rpc_url()->anyhow::Result<String> {
//     let result = ProjectArg::get_near_env().await.unwrap_or("nearenv");
//
// }
