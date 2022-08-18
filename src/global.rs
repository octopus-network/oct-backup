sea_query::sea_query_driver_postgres!();

use std::env;
use anyhow::Result;
use lazy_static::lazy_static;
use near_jsonrpc_client::{JsonRpcClient, NEAR_MAINNET_RPC_URL, NEAR_TESTNET_RPC_URL};
use async_once::AsyncOnce;
use sqlx::{PgPool, Pool, Postgres};
use crate::cmd::{Cmd, NearEnv};
use structopt::StructOpt;
use tracing::info;

lazy_static! {
    pub static ref CMD_ARG: AsyncOnce<Cmd> = AsyncOnce::new(async {
        Cmd::from_args()
    });
    pub static ref SYSTEM_ENV: AsyncOnce<SystemEnv> = AsyncOnce::new(async {
        init_env_config().expect("Failed to init env config.")
    });

    pub static ref NEAR_ENV: AsyncOnce<NearEnv> = AsyncOnce::new(async {
        let cmd = CMD_ARG.get().await;
        if cmd.near_env.is_some() {
            return cmd.near_env.clone().unwrap();
        }
        let env = SYSTEM_ENV.get().await;
        if env.near_env.is_some() {
            return env.near_env.clone().unwrap().parse().unwrap();
        }
        // default
        NearEnv::testnet
    });

    pub static ref NEAR_RPC_URL: AsyncOnce<String> = AsyncOnce::new(async {
       let cmd_arg = CMD_ARG.get().await;

        if cmd_arg.node_url.is_some() {
            cmd_arg.node_url.clone().unwrap()
        } else {
            let near_env = NEAR_ENV.get().await;
            let env = SYSTEM_ENV.get().await;
            match near_env {
                NearEnv::testnet => {
                    env.near_cli_testnet_rpc_server_url.clone().unwrap_or(NEAR_TESTNET_RPC_URL.to_string())
                }
                NearEnv::mainnet => {
                    env.near_cli_mainnet_rpc_server_url.clone().unwrap_or(NEAR_MAINNET_RPC_URL.to_string())
                }
            }
        }
    });
    pub static ref DEFAULT_NEAR_JSON_RPC_CLIENT: AsyncOnce<JsonRpcClient> =AsyncOnce::new( async {
        let near_rpc_url = NEAR_RPC_URL.get().await;
        println!("init near_rpc_url: {}", near_rpc_url);
        JsonRpcClient::connect(near_rpc_url)
    });

    pub static ref DATABASE_URL: AsyncOnce<Option<String>> = AsyncOnce::new(async {
        let cmd = CMD_ARG.get().await;
        if cmd.database_url.is_some() {
            return cmd.database_url.clone();
        }
        let database_url = SYSTEM_ENV.get().await.database_url.clone();
        info!("init database_url: {:?}", database_url);
        database_url
    });
    pub static ref DB_POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new( async {
        let database_url = DATABASE_URL.get().await.as_ref().expect("Failed to get database_url when init DB_POOL.");
        PgPool::connect(database_url)
        .await
        .unwrap()
    });
}


pub struct SystemEnv {
    pub(crate) near_env: Option<String>,
    pub(crate) database_url: Option<String>,
    pub(crate) rust_log: Option<String>,
    pub(crate) near_cli_testnet_rpc_server_url: Option<String>,
    pub(crate) near_cli_mainnet_rpc_server_url: Option<String>,
}

fn init_env_config() -> Result<SystemEnv> {
    Ok(SystemEnv {
        near_env: env::var("NEAR_ENV").ok(),
        database_url: env::var("DATABASE_URL").ok(),
        rust_log: env::var("RUST_LOG").ok(),
        near_cli_testnet_rpc_server_url: env::var("NEAR_CLI_TESTNET_RPC_SERVER_URL").ok(),
        near_cli_mainnet_rpc_server_url: env::var("NEAR_CLI_MAINNET_RPC_SERVER_URL").ok(),
    })
}