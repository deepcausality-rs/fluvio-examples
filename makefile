# Make will use bash instead of sh
SHELL := /usr/bin/env bash

.PHONY: help
help:
	@echo ' '
	@echo '    make build   	Builds the code base incrementally (fast) for dev.'
	@echo '    make check   	Checks the code base for security vulnerabilities.'
	@echo '    make fix   		Fixes linting issues as reported by clippy.'
	@echo '    make import   	Imports tick data from CSV into QuestDB.'
	@echo '    make format   	Formats call code according to cargo fmt style.'
	@echo '    make setup   	Tests and installs all make script dependencies.'
	@echo '    make run   		Runs the binary defined in scripts/run.sh.'
	@echo '    make update   	Update rust, pull latest from git remote, and build the project.'
	@echo '    make test   		Tests across all crates.'
	@echo '    make sbe   		Generates Rust bindings for SBE messages define in spec/sbe.'

# "---------------------------------------------------------"
# "---------------------------------------------------------"

.PHONY: build
build:
	@source scripts/build.sh


.PHONY: bench
bench:
	@source scripts/bench.sh


.PHONY: check
check:
	@source scripts/check.sh


.PHONY: clean
clean:
	@source scripts/clean.sh


.PHONY: db
db:
	@source scripts/db.sh


.PHONY: import
import:
	@source scripts/import.sh


.PHONY: fix
fix:
	@source scripts/fix.sh


.PHONY: format
format:
	@source scripts/format.sh


.PHONY: setup
setup:
	@source scripts/install_deps.sh


.PHONY: docker
docker:
	@source scripts/docker.sh


.PHONY: release
release:
	@source scripts/release.sh


.PHONY: run
run:
	@source scripts/run.sh


.PHONY: update
update:
	@source scripts/update.sh


.PHONY: test
test:
	@source scripts/test.sh


.PHONY: sbe
sbe:
	@source scripts/sbe.sh
