## Overview
Project template from cw-plus (for cw20 token) and cw-nft (for both cw721 & cw 1155)
and add or remove some feature to suit or Hopeverse use.

https://github.com/CosmWasm/cw-plus
https://github.com/CosmWasm/cw-nfts

## Installation.
``` 
git clone https://github.com/sukrit1234/HopeVerse-Cosomos.git HopeverseCosmos 
chmod -R 777 ./HopeverseCosmos
cd ./HopeverseCosmos
./init.sh
``` 

## Some tools
You also install build and optimize script tool for cargo here.
https://github.com/sukrit1234/cosmwasm-tool-bash


## Contracts
We provide some contracts to implements and use in our games and other Hopeverse project

Fungible Tokens:
- [`cw20-base`](./contracts/cw20-base) implementation of the cw20 spec (like ERC20 token on EVM) this is base of all fungible token on Hopeverse eco system.

Non Fungible Tokens:
- [`cw721-base`](./contracts/cw721-base) implementation of the cw721 spec (like ERC721 token on EVM) this is base of all non fungible token. That all of token has unique "token_id" 1 token 1 id, and token_id auto-generated when it minted.

- [`cw1155-base`](./contracts/cw1155-base) implementation of the cw1155 spec (like ERC1155 token on EVM) this is base of all non fungible multi token. Token must be define and generate "token_id" afterthat it can be minted. 1 token_id can hold by multiple account (that why we call multitoken). It's look like many of cw20-base token in one contract. But each token_id of token will hold different property - It's hybride between fungible token and non fungible token.