use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug,Clone)]
    pub enum NearEnv {
        testnet,
        mainnet
    }
}

arg_enum! {
    #[derive(Debug,Clone)]
    pub enum LogLevel {
        TRACE,
        DEBUG,
        INFO,
        WARN,
        ERROR,
    }
}

/// The options
#[derive(StructOpt, Debug)]
#[structopt(name = "backup")]
pub struct Cmd {
    /// near account id which deploy anchor contract
    pub contract: String,
    #[structopt(short)]
    /// input postgres database connection url, if not input, it will try to read from env variable: DATABASE_URL
    #[structopt(long = "database_url", short = "d")]
    pub database_url: Option<String>,
    #[structopt(subcommand)]
    pub sub_cmd: BackupType,
    /// input near env, it should be testnet or mainnet, if not input, it will try to read from env variable: NEAR_ENV
    #[structopt(long = "near_env", short = "n", possible_values = & NearEnv::variants(), case_insensitive = true)]
    pub near_env: Option<NearEnv>,
    /// NEAR node URL
    #[structopt(long = "node_url", short = "u")]
    pub node_url: Option<String>,
    /// Log level.
    #[structopt(long = "log_level", short = "l", possible_values = & LogLevel::variants(), case_insensitive = true)]
    pub log_level: Option<LogLevel>,
}

/// A backup tool of octopus-network's anchor contract.
#[derive(Debug, StructOpt)]
#[structopt(name = "BackupType")]
pub enum BackupType {
    /// back up validator set data
    ValidatorSet {
        /// appchain era
        era: u64,
        /// quantity of validator set need to backup from era
        #[structopt(default_value = "1")]
        quantity: u64,
        /// skip when sql save conflict
        #[structopt(long = "skip", short)]
        skip: bool,
    },
    /// back up staking history
    StakingHistory {
        /// staking history will backup from start_index
        start_index: u64,
        /// quantity of staking history need to backup from era
        #[structopt(default_value = "1")]
        quantity: u64,
    },
}
