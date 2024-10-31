# EarthMind

A Near Native implementation of the EarthMind Protocol.

EarthMind is a protocol for decentralized governance using advanced AI and Blockchain technology to streamline governance decisions for any protocol implementing the EarthMind client contract.

## Pre Requisites

- rust:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Near CLI RS
```
cargo install near-cli-rs
```

- cargo-near
```
cargo install cargo-near
```

- Near workspaces
```
cargo add near-workspaces
```

## Getting Started

```bash
$ git clone https://github.com/Machinalabs/earthmind-rs.git

$ cargo test  # run tests

$ cargo build # build project locally
```

## Deployment

To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# Create a new account
cargo near create-dev-account

# Deploy the contract on it
cargo near deploy <account-id>
```
## Earthmind Near Client

- [Client implementation](https://github.com/hasselalcala/earthmind-near-client)

## Useful Links
- [Rust](https://www.rust-lang.org/learn) - Documentation
- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://docs.near.org/tools/near-cli) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
