# silent-payment-indexer
This is a classroom project aimed at increasing the understanding of Silent Payments.

Silent Payments help keep your Bitcoin balances more private. Today, if people send you Bitcoin using one of your Bitcoin addresses, all that money ends up grouped together on the public blockchain. That means people know how much money you received. Silent Payments make it so the money does not get grouped together. It is possible to manually avoid getting your money grouped together, but that requires making a new Bitcoin address every time someone pays you. Silent Payments do that automatically.

This indexer helps with Silent Payments by searching through the blockchain for your money. It needs a secret from you to perform the search, and I recommend not provding that secret unless you know what you are doing. Afterall, this is just a classroom project.

# WIP Items
- [ ] Add all tagged hashes
- [ ] Run all recommended tests against pubkey extraction
- [x] Extract pubkeys from relevant txn type w/ basic testing
  - [x] p2pkh
  - [x] p2sh-p2wpkh
  - [x] p2wpkh
  - [x] p2tr

# Questions
- Labels for change: Will the m=0 labeled addresses look like a change addresses to nosy observers?
- Does declaring H as the blessed Nothing Up My Sleeves (NUMS) number reveal too much info? What if we did NUMS + hash

# Diagram

```
 ┏━━━━━━━━━━┱────────────╮           An outpoint - the transaction hash and its
 ┃ Outpoint ┃ txid, vout │           specific vout index.
 ┗━━━━━━━━━━┹────────────╯
 ┏━━━━━━━━━━┱────────────────────╮
 ┃ Sig Data ┃ scriptSig, witness │
 ┗━━━━━━━━━━┹────────────────────╯
 ┏━━━━━━━━━━┱──────────────────────╮ Trannsaction Output. The private key must
 ┃ TxOut    ┃ amount, scriptPubKey │ be able to sign the public key contained
 ┗━━━━━━━━━━┹──────────────────────╯ within.

```
