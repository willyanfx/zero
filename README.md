# Zero




## Notes

### CI
```shell
cargo watch -x check
```
or
```shell
cargo watch -x check -x test -x run
```

- It will start by running cargo check.
- If it succeeds, it launches cargo test.
- If tests pass, it launches the application with cargo run.

code coverage of a Rust project is via `cargo tarpaulin`

```shell
cargo tarpaulin --ignore-tests
```

#### Formating
```shell
cargo fmt -- --check
```

#### Audit 
```shell
cargo audit
```