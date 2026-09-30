# Payment Channels

Bidirectional payment channel example with off-chain state updates and final settlement.

## Use Cases
- Instant micro-payments between two parties
- Recurring payments without per-payment tx fees
- Scalable payment hub via off-chain balance updates

## Functions
- `init(token, participant_a, participant_b, pubkey_a, pubkey_b, expiry)` - Set up the channel. Both participants must authorize. `pubkey_*` are the ed25519 keys each participant uses to sign off-chain state.
- `deposit(from, amount)` - Fund the channel (participant auth required)
- `submit_state(from, balance_a, balance_b, sequence, sig_a, sig_b)` - Update balances with both parties' signatures (participant auth required)
- `close(from)` - Pay out both balances and close the channel (participant auth required)
- `get_info()` - Read the current channel state

## Signed state format
Both participants sign the same bytes:
`contract_address.to_xdr() || balance_a (i128 BE) || balance_b (i128 BE) || sequence (u32 BE)`.
Including the contract address prevents a state from being replayed on another channel. The sequence must strictly increase, and the balances must add up to the deposited total.

## Tests
The unit tests in `src/test.rs` are wired from `lib.rs` via `#[cfg(test)] mod test;`:

```bash
cargo test -p payment-channels
```
