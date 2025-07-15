# Licensed under the Apache License, Version 2.0 or the MIT License.
# SPDX-License-Identifier: Apache-2.0 OR MIT
# Copyright OXIDOS AUTOMOTIVE 2024.

.PHONY: ci-job-format
ci-job-format:
	@echo "Checking formatting of source files..."
	@./tools/run_fmt_check.sh

.PHONY: ci-job-clippy
ci-job-clippy:
	@echo "Running clippy on source files..."
	@./tools/run_clippy.sh

.PHONY: ci-job-mdbook
ci-job-mdbook:
	@echo "Generating book.toml..."
	@echo "Installing mdBook"
	@which mdbook >/dev/null 2>&1 || cargo install mdbook --locked
	@echo "Building docs"
	@./tools/gen_book_toml.sh ci
	@mdbook build docs

.PHONY: ci-runner-github
ci-runner-github: ci-job-format ci-job-clippy ci-job-mdbook
	@echo "Running cargo check..."
	@cargo check
	@echo "Running tests..."
	@cargo test


