# Actix Tom

Mssql/Mysql Http Server for Actix Web

## Usage

### server

```sh
cargo run

# Started http server: 127.0.0.1:8086
```

### web client

With [Postman](https://www.getpostman.com/) or [Rested](moz-extension://60daeb1c-5b1b-4afd-9842-0579ed34dfcb/dist/index.html)

- POST / (embed serde-json):

  - method : `POST`
  - url : `http://127.0.0.1:8080/`
  - header : `Content-Type` = `application/json`
  - body (raw) : `{"name": "Test user", "number": 100}`

- POST /manual (manual serde-json):

  - method : `POST`
  - url : `http://127.0.0.1:8080/manual`
  - header : `Content-Type` = `application/json`
  - body (raw) : `{"name": "Test user", "number": 100}`

- POST /mjsonrust (manual json-rust):

  - method : `POST`
  - url : `http://127.0.0.1:8080/mjsonrust`
  - header : `Content-Type` = `application/json`
  - body (raw) : `{"name": "Test user", "number": 100}` (you can also test `{notjson}`)

### python client

- `pip install aiohttp`
- `python client.py`

if ubuntu :

- `pip3 install aiohttp`
- `python3 client.py`

### build release

```sh
cargo build --release

nohup ./target/release/actix-tom > actix-tom.out 2>&1 &
```

### cross compiling

```sh
rustup target list
rustup target add x86_64-unknown-linux-musl
```

.cargo/config

```
[target.x86_64-unknown-linux-musl] 
linker = "x86_64-linux-musl-gcc"
```

Maybe need to update openssl and musl-tools

```sh
# update openssl
brew update
brew install openssl
brew link --force openssl

brew install FiloSottile/musl-cross/musl-cross
```

Cross-compiling Rust From Mac to Linux

```sh
TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
```
