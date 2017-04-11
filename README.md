# ios7crypt-rs: IOS7Crypt in Rust

# EXAMPLES

```console
$ ios7crypt -e monkey
1308181c00091d

$ ios7crypt -d 060b002f474b10
monkey

$ ios7crypt -h
Usage: ios7crypt [options]

Options:
    -h, --help          print usage info
    -v, --version       print version info
    -e, --encrypt VAL   encrypt a password
    -d, --decrypt VAL   decrypt a hash
```

# DOWNLOADS

https://github.com/mcandre/ios7crypt-rs/releases

# CRATE

https://crates.io/crates/ios7crypt

# API DOCUMENTATION

https://docs.rs/releases/search?query=ios7crypt

# REQUIREMENTS

* [Rust](http://www.rust-lang.org/) 1.16.0+

## Optional

* [Docker](https://www.docker.com)

# COMPILE AND INSTALL

```console
$ cargo install
```

# COMPILE FOR HOST

```console
$ cargo build

$ file target/debug/ios7crypt
target/debug/ios7crypt: Mach-O 64-bit executable x86_64
```

# CROSS-COMPILE

```console
$ docker pull mcandre/docker-rustup:x86_64-gnu
$ docker pull mcandre/docker-rustup:i686-gnu
$ docker pull mcandre/docker-rustup:x86_64-musl
$ docker pull mcandre/docker-rustup:i686-musl

$ make port

$ unzip -tv ios7crypt-*.zip
Archive:  ios7crypt-0.0.5.zip
    testing: target/x86_64-unknown-linux-gnu/release/ios7crypt   OK
    testing: target/i686-unknown-linux-gnu/release/ios7crypt   OK
    testing: target/x86_64-unknown-linux-musl/release/ios7crypt   OK
    testing: target/i686-unknown-linux-musl/release/ios7crypt   OK
No errors detected in compressed data of ios7crypt-0.0.5.zip.
```

# UNIT TEST

```console
$ cargo test
```

# PUBLISH CRATE

```console
$ cargo publish
```
