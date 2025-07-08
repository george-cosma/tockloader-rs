# Licensed under the Apache License, Version 2.0 or the MIT License.
# SPDX-License-Identifier: Apache-2.0 OR MIT
# Copyright OXIDOS AUTOMOTIVE 2024.

.PHONY: ci-job-format
ci-job-format:
	@echo "Checking formating of source files..."
	@./tools/run_fmt_check.sh

.PHONY: ci-job-clippy
ci-job-clippy:
	@echo "Running clippy on source files..."
	@./tools/run_clippy.sh

.PHONY: ci-job-mdbook
ci-job-mdbook:
	@echo "Installing mdBook if necessary and building docs..."
	@cargo install mdbook --version "^0.4" --force
	@mdbook build docs

.PHONY: ci-runner-github
ci-runner-github: ci-job-format ci-job-clippy
	@echo "Running cargo check..."
	@cargo check
	@echo "Running tests..."
	@cargo test
