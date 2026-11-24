# Ipê

A Rust behavior tree library with Serde serialization and typed, configurable leaves.
Build trees with sequences, selectors, inverters, and custom leaves; compile them
through a registry and tick them against your world, state, actions, and blackboard.

## Usage

Implement `Leaf` for your behaviors, register them with `Registry`, then compile a
`Tree` and call `CompiledTree::tick`. Trees can be loaded from JSON using
`serde_json`. See [the basic example](examples/basic.rs) for a complete walkthrough.

```sh
cargo run --example basic
cargo test
```

## License

[MIT](LICENSE).
