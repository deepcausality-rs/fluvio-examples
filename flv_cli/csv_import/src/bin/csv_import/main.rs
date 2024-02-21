use std::process;

mod process_file;
mod run;

/// Application entry point.
///
/// Calls the run() function from the run module.
///
/// If run() returns an error, prints the error and exits with
/// non-zero code.
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// Calls process::exit()
///
fn main() {
    if let Err(e) = run::run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
