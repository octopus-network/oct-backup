use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;
use serde::{Deserialize, Serialize};

arg_enum! {
    #[derive(Debug,Clone)]
    pub enum NearEnv {
        testnet,
        mainnet
    }
}

/// The options
#[derive(StructOpt, Debug)]
#[structopt(name = "backup")]
pub struct Cmd {
    pub contract: String,
    #[structopt(short)]
    pub database_url: Option<String>,
    #[structopt(subcommand)]
    pub sub_cmd: BackupType,
    #[structopt(long="near_env", short="n", possible_values = &NearEnv::variants(), case_insensitive = true)]
    pub near_env: Option<NearEnv>,
    /// NEAR node URL
    pub node_url: Option<String>
}

/// A backup tool of octopus-network's anchor contract.
#[derive(Debug, StructOpt)]
#[structopt(name = "BackupType")]
pub enum BackupType {
    ValidatorSet {
        /// appchain era
        era: u64,
        /// quantity of validator set need to backup from era
        #[structopt(default_value = "1")]
        quantity: u64,
        /// skip when sql save conflict
        #[structopt(long="skip", short)]
        skip: bool
    },
    #[structopt(about = "backup staking history")]
    StakingHistory {
        /// staking history will backup from start_index
        start_index: u64,
        /// quantity of staking history need to backup from era
        #[structopt(default_value = "1")]
        quantity: u64,
    }
}
