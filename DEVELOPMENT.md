# OVERVIEW

ios7crypt's own compilation process is compatible with standard cargo. We wrap some common workflows with tinyrick tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [Docker](https://www.docker.com/)

# INSTALL FROM LOCAL SOURCE

```console
$ tinyrick install
```

# UNINSTALL

```console
$ tinyrick uninstall
```

# LINT

```console
$ tinyrick lint
```

# UNIT + INTEGRATION TEST

```console
$ tinyrick test
```

# UNIT TEST

```console
$ tinyrick unit_test
```

# INTEGRATION TEST

```console
$ tinyrick integration_test
```

# GENERATE DOCUMENTATION

```console
$ tinyrick doc
```

# GENERATE PORTS

```console
$ tinyrick port
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```