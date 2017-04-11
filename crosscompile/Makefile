.PHONY: ios7crypt-rs-build-bot-gnu

all: ios7crypt-rs-build-bot-gnu

ios7crypt-rs-build-bot-gnu: gnu/Dockerfile
	sh -c "cd gnu && docker build -t mcandre/ios7crypt-rs-build-bot:gnu ."
