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
the `Developing.md` and `Publishing.md` files as useful references, but please set some
proper description in the README.


Attention pour utiliser le noeus cosmowasm, il faut rust 1.81
rustup default 1.81

Pour encoder le fichier
cargo wasm

attention à bien faire tourner source .profile

pour integrer le contrat au réseau
wasmd tx wasm store "./target/wasm32-unknown-unknown/release/sm01.wasm" \
  --from alice \
  --gas 2500000 \
  -y \
  --chain-id=docs-chain-1 \
  -o json \
  --keyring-backend=test


Pour récupérer le code ID
(mettre code de la transaction)
wasmd q tx 1DB2A8F71AD446AE96543016E6BA3737A87BD6EB690BDCA8177A270AF610FB34 -o json

A partir du CODE_ID dans events store_code
wasmd q wasm code $CODE_ID downloaded_code.wasm


# Retrieve the address of Alice's account
ALICE_ADDR=$(wasmd keys show alice -a --keyring-backend=test)
 
# Retrieve the address of Bob's account
BOB_ADDR=$(wasmd keys show bob -a --keyring-backend=test)
 
# Define init parameters for the contract
INIT="\"zero\""


wasmd tx wasm instantiate "$CODE_ID" "$INIT" \
  --admin="$ALICE_ADDR" \
  --from alice \
  --amount="100stake" \
  --label "local0.1.0" \
  --gas 1000000 \
  -y \
  --chain-id=docs-chain-1 \
  -o json \
  --keyring-backend=test

  récupérer adresse du contrat

  wasmd query wasm list-contract-by-code "$CODE_ID" -o json

  récupérer métadata contract
  wasmd q wasm contract $CONTRACT -o json


  MSG="\"inc\""
  MSG="\"dec\""
  MSG="\"set(5)\""      ça ne fonctionne pas

  exécuter
  wasmd tx wasm execute "$CONTRACT" "$MSG" \
  --from alice \
  --gas 1000000 \
  -y \
  --chain-id=docs-chain-1 \
  -o json \
  --keyring-backend=test


  consulter
  wasmd query wasm contract-state all "$CONTRACT" -o json

  la valeur est dans models[0].value en base64

  wasmd query wasm contract-state all "$CONTRACT" -o json | jq -r '.models[0].value' | base64 -d