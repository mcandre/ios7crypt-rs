BIN=target/debug/ios7crypt

all: test

test: $(BIN)
	$(BIN) -e monkey
	$(BIN) -d 060b002f474b10

$(BIN): src/bin/ios7crypt.rs
	cargo build

target/x86_64-unknown-linux-gnu/release/ios7crypt:
	sh -c "cd crosscompile/gnu && sh compile.sh"

target/x86_64-unknown-linux-musl/release/ios7crypt:
	sh -c "cd crosscompile/musl && sh compile.sh"

crosscompile: target/x86_64-unknown-linux-gnu/release/ios7crypt target/x86_64-unknown-linux-musl/release/ios7crypt

clean:
	-rm -rf target
