use std::process;

mod run;

fn main() {
    if let Err(e) = run::run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
