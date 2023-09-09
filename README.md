# CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of Rust and Cargo installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

**Latest**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
```

For cloning minimal code repo:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME -d minimal=true
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful referenced, but please set some
proper description in the README.

## Development

### To build the contract

1. Clean the cargo cache
   `cargo clean`
2. Get the dependencies
   `cargo update`
3. Additional dependency
   `rustup target add wasm32-unknown-unknown`
4. Build the contract
   `cargo wasm`

### Deploy the contract

1. Add it in the store

```
../sei-chain/build/seid tx wasm store artifacts/hackathon_movie_magic_contract.wasm -y --from=kanika-compass-testnet --chain-id=atlantic-2 --gas=1800000 --fees=180000usei --broadcast-mode=block --node=https://sei-testnet-rpc.polkachu.com/
```

2. Fetch the code_id from the log events

```
logs:
- events:
  - attributes:
    - key: action
      value: /cosmwasm.wasm.v1.MsgStoreCode
    - key: module
      value: wasm
    - key: sender
      value: sei1jc78av795r4mlky5rhf5jrn4cmxppak59aetah
    type: message
  - attributes:
    - key: code_id
      value: "2789"
    type: store_code
  log: ""
  msg_index: 0
```

3. Instantiate the contract

```
../sei-chain/build/seid tx wasm instantiate 2789 '{"count": 0}' --from=kanika-compass-testnet --broadcast-mode=block --label "movie-magic-hackathon" --chain-id=atlantic-2 --gas=1800000 --fees=180000usei --admin=sei1jc78av795r4mlky5rhf5jrn4cmxppak59aetah -y --node=https://sei-testnet-rpc.polkachu.com/
```

4. Contract Address `sei13tt4kqcjh4s02scwd56eqsy47kfagkq89whycmuejjzvpzmvnh0qll089u`
