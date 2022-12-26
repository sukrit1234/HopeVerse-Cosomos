# Cw721 Basic

This is a basic implementation of a cw721 NFT contract. It implements
the [CW721 spec](../../packages/cw721/README.md) and is designed to
be deployed as is, or imported into other contracts to easily build
cw721-compatible NFTs with custom logic.

Implements:

- [x] CW721 Base
- [x] Metadata extension
- [x] Enumerable extension

## Implementation

The `ExecuteMsg` and `QueryMsg` implementations follow the [CW721 spec](../../packages/cw721/README.md) and are described there.
Beyond that, we make a few additions:

* `InstantiateMsg` takes name and symbol (for metadata), as well as a **Minter** address. This is a special address that has full 
power to mint new NFTs (but not modify existing ones)
* `ExecuteMsg::Mint{token_id, owner, token_uri}` - creates a new token with given owner and (optional) metadata. It can only be 

It requires all tokens to have defined metadata in the standard format (with no extensions). For generic NFTs this may often be enough.

If provided, it is expected that the _token_uri_ points to a JSON file following the [ERC721 Metadata JSON Schema](https://eips.ethereum.org/EIPS/eip-721).