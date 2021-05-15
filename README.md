![1](static/img/label.png)

# Lupt Chat
Chat app to talk in group or to strangers

## Running
Create a `config.json` file for configuring the server with all the required options like `salt` and `tenor_api` etc.
An example config file `config.json.example` has been provided with all currently supported options, one can copy the example file as `config.json`
and put appropriate values for the given options to get the config file ready for use.

```
lupt -a <interface address> -p <port> -s <static files dir> -c <config.json path>
```

Example:

```
lupt -a 0.0.0.0 -p 8080 -s ./static -c config.json
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
