# CW1155 Spec: Non Fungible Multiple Tokens

CW1155 is a specification for managing multiple tokens based on CosmWasm.
The name and design is based on Ethereum's ERC1155 standard.

The specification is split into multiple sections, a contract may only
implement some of this functionality, but must implement the base.

Fungible tokens and non-fungible tokens are treated equally, non-fungible tokens just have one max supply.

Approval is set or unset to some operator over entire set of tokens. (More nuanced control is defined in
[ERC1761](https://eips.ethereum.org/EIPS/eip-1761))

## Base

### Messages

`SendFrom{from, to, token_id, amount, msg}` - This transfers some amount of tokens between two accounts. If `to` is an
address controlled by a smart contract, it must implement the `CW1155Receiver` interface, `msg` will be passed to it
along with other fields, otherwise, `msg` should be `None`. The operator should either be the `from` account or have
approval from it.

`BatchSendFrom{from, to, batch: Vec<(token_id, amount)>, msg}` - Batched version of `SendFrom` which can handle multiple
types of tokens at once.

`Mint {to, token_id, amount}` - This mints some tokens to `to` account.

`BatchMint {to, batch: Vec<(token_id, amount)>}` - Batched version of `Mint`.

`Burn {from, token_id, amount}` - This burns some tokens from `from` account.

`BatchBurn {from, batch: Vec<(token_id, amount)>}` - Batched version of `Burn`.

`ApproveAll{ operator, expires }` - Allows operator to transfer / send any token from the owner's account. If expiration
is set, then this allowance has a time/height limit.

`RevokeAll { operator }` - Remove previously granted ApproveAll permission

### Queries

`Balance { owner, token_id }` - Query the balance of `owner` on particular type of token, default to `0` when record not
exist.

`BatchBalance { owner, token_ids }` - Query the balance of `owner` on multiple types of tokens, batched version of `Balance`.

`AllBalance { owner }` - Query all (token_id,amount) of `owner` default empty array when record not exist.

`ApprovedForAll {owner, include_expired, start_after, limit}` - List all operators that can access all of the owner's
tokens. Return type is `ApprovedForAllResponse`.  If `include_expired` is set, show expired owners in the results,
otherwise, ignore them.

`Allowance{owner, operator}` - Query approved status `owner` granted to `operator`. Return type is `AllowanceResponse`.

### Receiver

Any contract wish to receive CW1155 tokens must implement `Cw1155ReceiveMsg` and `Cw1155BatchReceiveMsg`.

`Cw1155ReceiveMsg { operator, from, token_id, amount, msg}` - 

`Cw1155BatchReceiveMsg { operator, from, batch, msg}` - 

## Metadata

### Queries

`TokenInfo{ token_id }` - Query metadata url of `token_id`.

## Enumerable

### Queries

Pagination is acheived via `start_after` and `limit`. Limit is a request
set by the client, if unset, the contract will automatically set it to
`DefaultLimit` (suggested 10). If set, it will be used up to a `MaxLimit`
value (suggested 30). Contracts can define other `DefaultLimit` and `MaxLimit`
values without violating the CW1155 spec, and clients should not rely on
any particular values.

If `start_after` is unset, the query returns the first results, ordered by
lexogaphically by `token_id`. If `start_after` is set, then it returns the
first `limit` tokens *after* the given one. This allows straight-forward 
pagination by taking the last result returned (a `token_id`) and using it
as the `start_after` value in a future query. 

`Tokens{owner, start_after, limit}` - List all token_ids that belong to a given owner.
Return type is `TokensResponse{tokens: Vec<token_id>}`.

`AllTokens{start_after, limit}` - Requires pagination. Lists all token_ids controlled by the contract.
