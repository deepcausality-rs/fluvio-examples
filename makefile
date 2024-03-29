# Make will use bash instead of sh
SHELL := /usr/bin/env bash

.PHONY: help
help:
	@echo '    make qdgw   	Start the Start the Quant Data Gateway (QDGW).'
	@echo '    make symdb   	Start the Symbol Master Database Service (SYMDB)'
	@echo '    make example     	Run the example code in flv_examples.'
#
	@echo '    make build   	Builds the code base incrementally (fast) for dev.'
	@echo '    make check   	Checks the code base for security vulnerabilities.'
	@echo '    make doc   		Builds, tests, and opens api docs in a browser.'
	@echo '    make fix   		Fixes linting issues as reported by clippy.'
	@echo '    make import   	Imports tick data from CSV into QuestDB.'
	@echo '    make format   	Formats call code according to cargo fmt style.'
	@echo '    make setup   	Tests and installs all make script dependencies.'
	@echo '    make run   		Runs the default binary (QDGW).'
	@echo '    make update   	Update rust, update and build the project.'
	@echo '    make test   	Tests across all crates.'
	@echo '    make sbe   		Generates Rust bindings from the SBE schema.'

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


.PHONY: doc
doc:
	@source scripts/doc.sh


.PHONY: example
example:
	@source scripts/example.sh


.PHONY: fix
fix:
	@source scripts/fix.sh


.PHONY: import
import:
	@source scripts/import.sh


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


.PHONY: qdgw
qdgw:
	@source scripts/qdgw.sh


.PHONY: symdb
symdb:
	@source scripts/symdb.sh


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
