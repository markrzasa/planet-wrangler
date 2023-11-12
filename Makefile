THIS_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

install-clippy:
	rustup update && rustup component add clippy

lint:
	cargo clippy
