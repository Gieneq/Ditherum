# Ditherer

## Tests & Logging
To run test with logging option

> Log levels: error > warn > info > debug > trace.

Windows:
```cmd
set RUST_LOG=debug && cargo test --features logging -- --nocapture
```

Linux:
```sh
RUST_LOG=debug && cargo test --features logging -- --nocapture
```


