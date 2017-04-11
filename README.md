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

# BUILD AND INSTALL

```console
$ cargo install
```

# BUILD

```console
$ cargo build
```

# UNIT TEST

```console
$ cargo test
```

# PUBLISH CRATE

```console
$ cargo publish
```
