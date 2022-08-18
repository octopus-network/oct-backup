# Anchor-BackUp

A command-line tool for backing up the validator set data and staking action data in anchor contract.  

# How to use

You can see how to use it by `-h`:

```shell
USAGE:
    backup [OPTIONS] <contract> [node-url] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <database-url>            input postgres database connection url
    -n, --near_env <near-env>     [possible values: testnet, mainnet]

ARGS:
    <contract>    near account id which deploy anchor contract
    <node-url>    NEAR node URL

SUBCOMMANDS:
    help               Prints this message or the help of the given subcommand(s)
    staking-history    back up staking history
    validator-set      back up validator set data

```

