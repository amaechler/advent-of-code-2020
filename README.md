# Advent of Code 2020

## Environment Setup

### Windows Subsystem for Linux 2

Besides setting up WSL2, make sure to install the `build-essentials` package:

```sh
sudo apt install build-essential
```

### Rust

Install Rust in WSL2 as outlined [here](https://rustup.rs/):

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
[...]

# restart shell to load updated profile
$ rustc --version
rustc 1.48.0 (7eac88abb 2020-11-16)
```

### Visual Studio Code

In `vscode`, I installed the extensions `rust-lang.rust` and `ms-vscode-remote.remote-wsl`.

## How to Run

```sh
$ cargo build
   Compiling advent-of-code-2020 v0.1.0 (/mnt/c/Users/amaechler/Development/GitHub/_Personal/advent-of-code-2020)
    Finished dev [unoptimized + debuginfo] target(s) in 1.09s

$ cargo run day1
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/advent-of-code-2020`
```
