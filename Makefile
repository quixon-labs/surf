SHELL := /bin/bash
CARGO_TOOLCHAIN ?= nightly
CARGO_ARGS ?=

export DEBUG ?= 0

.ONESHELL:

.PHONY: default
default: build

.PHONY: release
release: DEBUG=0
release: clean build

.PHONY: debug
debug: DEBUG=1
debug: build

.PHONY: dev
dev: DEBUG=1
dev: run

.PHONY: clean
clean:
	@$(call cargo,clean)	

.PHONY: build
build: 
	@$(call cargo,build)

.PHONY: run
run: 
	@$(call cargo,run)

.PHONY: check
check: 
	@$(call cargo,check --all-features)

.PHONY: publish
publish:
	@set -e
	cargo +$(CARGO_TOOLCHAIN) fmt
	cargo +$(CARGO_TOOLCHAIN) clippy
	cargo +$(CARGO_TOOLCHAIN) publish


define cargo =
set -e
cargo_args="$(CARGO_ARGS)"
if [[ "$(DEBUG)" != "1" ]]; then cargo_args="--release $(CARGO_ARGS)"; fi
cargo +$(CARGO_TOOLCHAIN) $(1) $${cargo_args}
endef