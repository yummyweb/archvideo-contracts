# ArchVideo Smart Contracts

There are two smart contracts in this repository - `archvideo` and `archvideo-nft`.

## Compilation and Deployment

First, ensure `archway` CLI is installed. Then, ensure an account is created and configured. To do so, follow the documentation here: https://docs.archway.io/developers/getting-started/setup#creating-an-account. You must also ensure that sufficient balance is in the account. To receive testnet tokens, follow the documentation here: https://docs.archway.io/developers/guides/faucet. Once it's installed and an account is created, execute the following commands.

```
$ archway contracts build
```

This step builds the contracts. Do ensure docker is running because without docker, this command will not run.

```
$ archway contracts store 'archvideo'
```

This step stores the contract on the blockchain. Seems needless, and it kinda is.

```
$ archway contracts instantiate archvideo --args '{}'
```

This step instantiates the contract. Refer to the `InstantiateMsg` for arguements. It is an entrypoint of the contract.

```
$ archway contracts execute archvideo --args '{"upload_video":{"video_hash":"some hash","issue":"some issue","title":"some title","id":"some id"}}'
```

This step executes the `Upload Video` function and essentially, uploads a video. The data supplied to the function has to follow the format for the `UploadVideo` struct in the `msg.rs` file.

```
$ archway contracts query smart archvideo --args '{"query_videos":{}}'
```

This step queries all the videos stored. It executes the `Query Videos` function.