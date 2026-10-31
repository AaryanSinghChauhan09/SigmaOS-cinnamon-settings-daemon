# SigmaOS Cinnamon settings daemon

This crate is currently a `no_std` identity scaffold. Its only public type
constructs a `Component` with the fixed name `SigmaOS Component`. It does not
read or apply desktop settings, watch for changes, or run a daemon.

The inline unit test checks only the fixed scaffold name. It is not evidence of
settings persistence or desktop integration.

## Build

```sh
cargo check --lib
```

## License

Apache-2.0. See [`Cargo.toml`](Cargo.toml).
