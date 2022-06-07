```
$ node --version
$ nvm install 14

$ rustc --version

$ sh -c "$(curl -sSfL https://release.solana.com/v1.10.23/install)"

$ solana config set --url localhost
Config File: /Users/yuta/.config/solana/cli/config.yml
RPC URL: http://localhost:8899
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /Users/yuta/.config/solana/id.json
Commitment: confirmed

$ solana-keygen new

Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none):

Wrote new keypair to /Users/yuta/.config/solana/id.json
===============================================================================
pubkey: EoVSJSEqWbmDBHm7h6xb7UsZWS56Bso1PbdZRQA1ra8q
```


```
npm install

npm run build:program-rust

cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program
BPF SDK: /Users/yuta/.local/share/solana/install/releases/1.10.24/solana-release/bin/sdk/bpf
cargo-build-bpf child: rustup toolchain list -v
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
error: command failed: 'cargo': Bad CPU type in executable (os error 86)
-> error
```

cargo build-bpf ... binary begins with cargo-build-bpf in $PATH is called
The actual command is here:
https://github.com/solana-labs/solana/blob/v1.10/sdk/cargo-build-bpf/src/main.rs


cargo +bpf ... bpf is a toolchain, + interpreted as toolchain
https://doc.rust-lang.org/cargo/commands/cargo-build.html#common-options

