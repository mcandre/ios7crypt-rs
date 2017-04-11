.PHONY: ios7crypt-$(VERSION).zip

VERSION=0.0.5

BIN=target/debug/ios7crypt

all: test

test: $(BIN)
	$(BIN) -e monkey
	$(BIN) -d 060b002f474b10

$(BIN): src/bin/ios7crypt.rs
	cargo build

target/x86_64-unknown-linux-gnu/release/ios7crypt:
	sh compile-x86_64-gnu.sh

target/i686-unknown-linux-gnu/release/ios7crypt:
	sh compile-i686-gnu.sh

target/x86_64-unknown-linux-musl/release/ios7crypt:
	sh compile-x86_64-musl.sh

target/i686-unknown-linux-musl/release/ios7crypt:
	sh compile-i686-musl.sh

crosscompile: target/x86_64-unknown-linux-gnu/release/ios7crypt target/i686-unknown-linux-gnu/release/ios7crypt target/x86_64-unknown-linux-musl/release/ios7crypt target/i686-unknown-linux-musl/release/ios7crypt

ios7crypt-$(VERSION).zip: crosscompile
	zip ios7crypt-$(VERSION).zip target/x86_64-unknown-linux-gnu/release/ios7crypt target/i686-unknown-linux-gnu/release/ios7crypt target/x86_64-unknown-linux-musl/release/ios7crypt target/i686-unknown-linux-musl/release/ios7crypt

port: ios7crypt-$(VERSION).zip

clean:
	-rm -rf target
