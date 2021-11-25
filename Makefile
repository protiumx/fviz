SHELL := /usr/bin/env bash
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
COMMANDS := build run test
FVIZ_SERVER_TOML = --manifest-path packages/fviz-server/Cargo.toml
.DEFAULT_GOAL := help

######################################################
# BASIC RULES
######################################################

.PHONY: help build-fviz-server build-fviz-ui test-fviz-server test-fviz-ui run-fviz-server run-fviz-ui
.PHONY: build run test

help: ## shows this output
	@awk 'BEGIN {FS = ":.*?## "} /([a-zA-Z_-]\s?){1,}:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build run test: ## (builds | runs | test) a package. usage: <command> package=<package_name>
ifdef package
	$(MAKE) $(@)-fviz-$(package)
else:
	$(error missing `package` variable)
endif

build-fviz-server: ## runs cargo build for fviz-server
	cargo build $(FVIZ_SERVER_TOML)

build-fviz-ui: ## runs yarn build for fviz-ui
	yarn --cwd packages/fviz-ui/ build

build-fviz-simulator: ## runs go build for fviz-simulator
	make -C packages/fviz-simulator/ build

run-fviz-server: ## runs cargo run for fviz-server
	cargo run $(FVIZ_SERVER_TOML)

run-fviz-ui: ## runs yarn start for fviz-ui
	yarn --cwd packages/fviz-ui/ start

run-fviz-simulator: ## runs go run for fviz-simulator
	make -C packages/fviz-simulator/ build

test-fviz-server: ## runs cargo test for fviz-server
	cargo test $(FVIZ_SERVER_TOML)

test-fviz-ui: ## runs yarn test for fviz-ui
	yarn --cwd packages/fviz-ui/ test

run-fviz-simulator: ## runs go test for fviz-simulator
	make -C packages/fviz-simulator/ test
