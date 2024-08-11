&nbsp;

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset=".github/.assets/casa-sns.icon.svg">
    <source media="(prefers-color-scheme: light)" srcset=".github/.assets/casa-sns.icon.svg">
    <img alt="We Are Sweet Logo" src="./github/.assets/was-logo.scheme-dark.svg" width="100">
  </picture>
</div>

## Casa Simple Notification Service

> A simple, self-hostable notification service for Algorand Virtual Machine blockchains

> [!WARNING] 
> This is a work in progress and not for use in production

### Install

- `cargo install cargo-watch`
- `cargo build`

### Run

```
cargo run -- --account RCN4XS53BDBHG76TDMIHUGSXGXRIBS3YKYCDYYK7NUERX6XLFOLFZCCCHI --network algorand --environment testnet
```

```
cargo watch -x "run -- --account RCN4XS53BDBHG76TDMIHUGSXGXRIBS3YKYCDYYK7NUERX6XLFOLFZCCCHI --network algorand --environment testnet"
```

> Run `cargo run` or `cargo watch -x run` for a demo

### Misc

#### TODO

- [ ] Device registration endpoint
- [ ] Device de-registration endpoint
- [ ] Watch account
- [ ] Message queue
- [ ] Persistent store for errors and retries (SeaORM)
- [ ] Notification services
  - [ ] APNS
  - [ ] Firebase Cloud Messaging
- [ ] ???
- [ ] (Non-)profit 🥹
