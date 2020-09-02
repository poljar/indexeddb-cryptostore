# indexeddb-cryptostore

A prototype [`CryptoStore`] implementation for the [`matrix-rust-sdk`] using
[`indexeddb`].

The tests can be run with

```bash
cargo test --target wasm32-unknown-unknown
```

[`matrix-rust-sdk`]: https://github.com/matrix-org/matrix-rust-sdk
[`CryptoStore`]: https://github.com/matrix-org/matrix-rust-sdk/blob/master/matrix_sdk_crypto/src/store/mod.rs#L104
[`indexeddb`]: https://github.com/poljar/indexeddb-rs
