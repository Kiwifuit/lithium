# Lithium
A program to snatch Chrome logins

- [Lithium](#lithium)
  - [Features](#features)
  - [Build Instructions](#build-instructions)
    - [Prerequisites](#prerequisites)
    - [Build Commands](#build-commands)


> **Info**: Project is highly in development and stuff might change, or said stuff might be untested

> **Note**: This can *only* ran on a Windows machine due
> to an OS-Specific encryption method. Support for Linux
> and Mac is appreciated but I won't be implementing that anytime soon

## Features
**For now, these features are all dead, nothing will change if you add them or not when building**

* `package-data`
  * Packages data into a `data.zip` file near the executable
* `discord-webhook`
  * Sends an HTTP POST to a webhook with the data harvested
  * Passwords sent will be decrypted but have a spoiler
  * **Will *not* check if it's gonna be `POST`ing to Discord. You can put any HTTP endpoint and it will always `POST` to that URL**

## Build Instructions
### Prerequisites
* `cargo`, `rustc` and `rustup`
* (on Linux/MacOS) the `x86_64-pc-windows-gnu` target, C compiler, and linker
* A copy of this repository

### Build Commands
```
cargo build --release
```
If you want to add features ([read the `Cargo.toml` for options](/Cargo.toml)), you can optionally add a `--features` flag:
```
cargo build --release --features package-data,discord-webhook
```
The output binary can be found at `./target/x86_64-pc-windows-gnu/release/`