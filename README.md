## About

`ngWallet CLI(Command Line Interface)` is a tool to access Ngin Network from the command line. It connects to an external node (_"upstream"_) and allows a user or application to read information from the blockchain and to send new transactions. In the latter case it provides functionality to sign transactions by a provided Private Key. The tool integrates Ngin Vault with the intention of generation, import, and/or storing of Ngin Private Keys.

## Usage

```shell
$ emerald --help

emerald
Command-line interface for Emerald platform

USAGE:
    emerald [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Display version

OPTIONS:
    -p, --base-path <base-path>    Set path for chain storage
    -c, --chain <chain>            Sets a chain name [default: mainnet]

SUBCOMMANDS:
    account        Account related commands
    balance        Request account's balance from node through RPC
    help           Prints this message or the help of the given subcommand(s)
    mnemonic       Create mnemonic phrase according to BIP39 spec
    server         Start local RPC server
    transaction    Transaction related commands

```


## Installing ngWallet CLI

### Download stable binary

Binaries for all platforms are currently published at https://github.com/NginProejct/ngWallet-cli/releases

### Build from sources

#### Requirements

Install Rust from https://www.rust-lang.org/en-US/install.html
  
Unix one-liner:
```
curl https://sh.rustup.rs -sSf | sh
```
  
On Windows, Rust additionally requires the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire
the build tools is by installing Microsoft Visual C++ Build Tools 2017 which provides just the Visual C++ build tools.
  
#### Compile

```
git clone https://github.com/NginProject/ngWallet-cli.git
cd ngWallet-cli
cargo build --release
cd target\debug
```

## License

Apache 2.0
