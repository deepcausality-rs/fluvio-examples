## 🛠️ Cargo & Make

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work. To do so, please run the following command:

```bash 
    make setup
```

The make install command tests and tries to install all required developer dependencies.
if the automatic install fails, the script will show a link with further installation instructions.

After all dependencies have been installed, the following commands are ready to use.

```
make
    make qdgw   	Start the Start the Quant Data Gateway (QDGW).
    make symdb   	Start the Symbol Master Database Service (SYMDB)
    make example    Run the example code in flv_examples.
    make build   	Builds the code base incrementally (fast) for dev.
    make check   	Checks the code base for security vulnerabilities.
    make doc   		Builds, tests, and opens api docs in a browser.
    make fix   		Fixes linting issues as reported by clippy.
    make import   	Imports tick data from CSV into QuestDB.
    make format   	Formats call code according to cargo fmt style.
    make setup   	Tests and installs all make script dependencies.
    make run   		Runs the default binary (QDGW).
    make update   	Update rust, update and build the project.
    make test   	Tests across all crates.
    make sbe   		Generates Rust bindings from the SBE schema.
```

The scripts called by each make command are located in the [script folder.](scripts)