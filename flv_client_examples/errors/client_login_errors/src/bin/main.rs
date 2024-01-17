use std::error::Error;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello, client_login_errors!");

    Ok(())
}
