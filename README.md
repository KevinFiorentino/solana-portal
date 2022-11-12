# Image portal - Solana program

### Fund Wallet

- `solana config set --url devnet`
- `solana airdrop 2` x2
- `solana balance`

### Deploy smart contract

- `anchor build`
- `solana address -k target/deploy/xxxxx-keypair.json`
- Replace Program ID in `lib.rs` and `Anchor.toml`
- `anchor build` (again)
- `anchor deploy`

### Test

- `anchor test`

[See the React front-end](https://github.com/KevinFiorentino/react-solana-portal)

---

Course by [buildspace](https://buildspace.so/p/build-solana-web3-app)
