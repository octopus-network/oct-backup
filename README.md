# Anchor-BackUp

A command-line tool for backing up the validator set data and staking action data in anchor contract.  

# How to use


1. Config `database_url` by command argument or environment variable. 
2. Use command to back up data, you can see more detail to use it by `-h`:

```shell
USAGE:
    oct-backup [OPTIONS] <contract> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --database_url <database-url>    input postgres database connection url, if not input, it will try to read from
                                         env variable: DATABASE_URL
    -l, --log_level <log-level>          Log level [possible values: TRACE, DEBUG, INFO, WARN, ERROR]
    -n, --near_env <near-env>            input near env, it should be testnet or mainnet, if not input, it will try to
                                         read from env variable: NEAR_ENV [possible values: testnet, mainnet]
    -u, --node_url <node-url>            NEAR node URL

ARGS:
    <contract>    near account id which deploy anchor contract

SUBCOMMANDS:
    help               Prints this message or the help of the given subcommand(s)
    staking-history    back up staking history
    validator-set      back up validator set data

```

# Example

## Validator set
1. How to back up `barnacle0918.registry.test_oct.testnet` validator set data of era 1:
```shell
export DATABASE_URL=<database-url>
oct-backup barnacle0918.registry.test_oct.testnet validator-set 1
```

2. How to back up `barnacle0918.registry.test_oct.testnet` validator set data of era 1 to 5:
```shell
export DATABASE_URL=<database-url>
oct-backup barnacle0918.registry.test_oct.testnet validator-set 1 5
```


## Staking history

1. How to back up `barnacle0918.registry.test_oct.testnet` staking histories data of index 1:
```shell
export DATABASE_URL=<database-url>
oct-backup barnacle0918.registry.test_oct.testnet staking-history 1
```

2. How to back up `barnacle0918.registry.test_oct.testnet` staking histories data of index 1 to 5:
```shell
export DATABASE_URL=<database-url>
oct-backup barnacle0918.registry.test_oct.testnet staking-history 1 5
```