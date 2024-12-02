# -----------------------------------------------------------------------------
# Make configuration
# -----------------------------------------------------------------------------
PROJECT = $(shell basename $(CURDIR))
.DEFAULT_GOAL = help

# -----------------------------------------------------------------------------
# Setup commands (## comments for description)
# -----------------------------------------------------------------------------
.PHONY: init
init: destroy build run ## Run this to setup and start the project from scratch (can be re-run to rebuild everything from scratch)

.PHONY: destroy
destroy: ## Delete the previous setup
	echo remove goes here

.PHONY: build
build: ## Build the project
	docker build -t aoc2024 .

.PHONY: run
run: ## Start the service
	docker run -it --rm --name aoc2024 aoc2024

# -----------------------------------------------------------------------------
# Development commands (##- comments for description)
# -----------------------------------------------------------------------------
.PHONY: shell
shell: ##- Run a bash shell
	${DCR} bash

.PHONY: test
test: ##- Run tests
	${DCR} echo test goes here

.PHONY: help
help: ##- Print this help text
	@printf "%s\n" "Setup commands:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m  make %-15s\033[0m %s\n", $$1, $$2}'
	@printf "%s\n" "Development commands:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?##- .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?##- "}; {printf "\033[36m  make %-15s\033[0m %s\n", $$1, $$2}'
