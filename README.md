![1](static/img/label.svg)

# Lupt Chat
Chat app to talk in group or to strangers

## Running
Export SALT as environment variable to use as salt for hashing, and TENOR_API_KEY for gifs support in the server.

```
SALT="<salt>" TENOR_API_KEY="API-KEY" lupt --bind_address <interface address>:<port> --static_path <static files dir>
```

Example:

```
SALT="sometext" TENOR_API_KEY="API-KEY" lupt --bind_address 0.0.0.0:8080 --static_path ./static
```

## Building

Make sure the rust toolchain is installed. (If not, install using [rustup.rs](https://rustup.rs/))
Then, just use cargo to build the project binary

```
cargo build --release
```

This will produce the required project binary `lupt` in `target/release/lupt`

## License

This project is under [GPLv3](LICENSE)
