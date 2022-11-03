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
#### health 
```shell
curl -v http://127.0.0.1:8000/health_check
```

### DB
#### Docker

```shell
docker pull postgres
```
We can then launch Postgres with `./scripts/init_db.sh`

```shell
chmod +x scripts/init_db.sh
```
#### sqlx-cli
provides a command-line interface,manage database mi- grations.
 
```shell
cargo install sqlx-cli --no-default-features --features postgres
```