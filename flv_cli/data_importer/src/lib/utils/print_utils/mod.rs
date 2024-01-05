use std::time::Duration;

pub fn print_import_header() {
    println!();
    println!("import_kraken: Imports trade tick data from CSV into QuestDB.");
    println!();
}

pub fn dbg_print(vrb: bool, msg: &str) {
    if vrb {
        println!("{msg}");
        println!();
    }
}

pub fn print_duration(elapsed: &Duration) {
    println!("Program took {:?} seconds.", elapsed.as_secs());
    println!();
}
