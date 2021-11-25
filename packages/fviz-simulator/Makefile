SHELL := /usr/bin/env bash
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

help: ## shows this output
	@awk 'BEGIN {FS = ":.*?## "} /([a-zA-Z_-]\s?){1,}:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## build the go app
	go build -o tmp/fviz-simulator ./cmd/drone/main.go

format: ## format the go app
	go fmt ./...

install: ## install the dependencies
	go install ./cmd/drone/main.go

run: build ## run the go app
	go run ./cmd/drone/main.go

test: ## execute tests and remove _mocks|_generated files from the coverage.
	go test -count=1 -race -timeout 10s ./...
