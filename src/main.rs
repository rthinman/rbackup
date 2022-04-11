//! Rust version of my fourth-generation backup utility
//! Backs up to hard drive, and meant to be run from command line.
//! 
//! Python version was designed to be cross platform (Windows, Linux),
//! but this one is presently only known to work on Windows.

use std::process;

fn main() {
    let args = rbackup::parse_args();
    if let Err(e) = rbackup::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
