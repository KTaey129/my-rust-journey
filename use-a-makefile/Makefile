# you can set customized command setting using makefile
# Why Use @?
# Cleaner Output: It hides the commands themselves, making the output more user-friendly.
# Focus on Output: For tasks where the command is less important than its result (e.g., echo, read, or printing a success message).
# Custom Feedback: Allows you to display only meaningful feedback, such as status messages, without showing the underlying shell commands.
# Best Practices
# Use @ for commands that display messages or handle user interaction.
# Avoid @ for critical build commands (like gcc or cargo build) during development, so you can see what's being executed if there’s an error.
SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bump: ## Bump the version number
	@current_version=$(shell cargo pkgid | cut -d# -f2); \
	echo "Current version is $$current_version"; \
	read -p "Enter new version number: " version; \
	if [[ ! $$version =~ ^[0-9]+\.[0-9]+\.[0-9]+$$ ]]; then \
		echo "Invalid version format. Use x.y.z"; exit 1; \
	fi; \
	sed -i -E "s/^version = .*/version = \"$$version\"/" Cargo.toml; \
	echo "New version is $$version"

doc: ## Generate documentation for the project using cargo
	cargo doc

test: ## Run tests in the project using cargo
	cargo test

release: ## Build the project in release mode
	cargo build --release

## The fi closes the if statement. The \ at the end of each line allows the entire block to be written in a single logical line in the Makefile.

bump-patch: ## Increment the patch version
	@current_version=$(shell cargo pkgid | cut -d# -f2); \
	new_version=$$(echo $$current_version | awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}'); \
	sed -i -E "s/^version = .*/version = \"$$new_version\"/" Cargo.toml; \
	echo "Updated patch version: $$new_version"

bump-minor: ## Increment the minor version
	@current_version=$(shell cargo pkgid | cut -d# -f2); \
	new_version=$$(echo $$current_version | awk -F. '{printf "%d.%d.%d", $$1, $$2+1, 0}'); \
	sed -i -E "s/^version = .*/version = \"$$new_version\"/" Cargo.toml; \
	echo "Updated minor version: $$new_version"

bump-major: ## Increment the major version
	@current_version=$(shell cargo pkgid | cut -d# -f2); \
	new_version=$$(echo $$current_version | awk -F. '{printf "%d.%d.%d", $$1+1, 0, 0}'); \
	sed -i -E "s/^version = .*/version = \"$$new_version\"/" Cargo.toml; \
	echo "Updated major version: $$new_version"	
