use std::process;

mod run;
mod handle;

fn main() {
    if let Err(e) = run::run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
