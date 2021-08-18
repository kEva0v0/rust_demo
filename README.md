# rust_demo

personal rust learning project

## install

```Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

then edit ~/.zshrc and insert

```Rust
export PATH="$PATH:~/.cargo/bin"
```

## Extenstion

- install code lint

```Rust
rustgo install nightly
cargo +nightly install racer
```

- backtrace

```Rust
export RUST_BACKTRACE=1
```