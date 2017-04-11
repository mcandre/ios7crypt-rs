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

$ make crosscompile

$ file target/x86_64-unknown-linux-gnu/release/ios7crypt
target/x86_64-unknown-linux-gnu/release/ios7crypt: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 2.6.32, BuildID[sha1]=d1cb7423e44172aeb754c827084ffde9edcafe91, not stripped
$ file target/i686-unknown-linux-gnu/release/ios7crypt
target/i686-unknown-linux-gnu/release/ios7crypt: ELF 32-bit LSB shared object, Intel 80386, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux.so.2, for GNU/Linux 2.6.32, BuildID[sha1]=6b25444b647e4be87907f4e4a2c983ee3136a4c4, not stripped
$ file target/x86_64-unknown-linux-musl/release/ios7crypt
target/x86_64-unknown-linux-musl/release/ios7crypt: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=9b3084a0e50b8d31693b5db7303c04d38e66b9f3, not stripped
```

# UNIT TEST

```console
$ cargo test
```

# PUBLISH CRATE

```console
$ cargo publish
```
