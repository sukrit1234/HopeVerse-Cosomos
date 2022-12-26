# CW721 Metadata URL

NFT creators may keep static and just store url of metadata that generate from IPFS.
With CW721-Base in CosmWasm.

```

In particular, the fields defined conform to the properties supported in the [OpenSea Metadata Standard](https://docs.opensea.io/docs/metadata-standards).


This means when you query `TokenInfo{token_id}`, you will get something like:

```json
{
  "name": "Enterprise",
  "token_uri": "https://starships.example.com/Starship/Enterprise.json",
}
```

Please look at the test code for an example usage in Rust.

## Notice

Feel free to use this contract out of the box, or as inspiration for further customization of cw721-base.
We will not be adding new features or business logic here.
