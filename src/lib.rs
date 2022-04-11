use clap::Parser;
use std::error::Error;
use std::fs;
use serde::{Serialize, Deserialize};

const DEFAULT_BACKUP_DAYS: u8 = 14;

#[derive(Parser, Debug)]
#[clap(name = "rbackup", author, version, about = "Rust version of my backup utility", long_about = None)]
pub struct Cli {
    /// Backup job filenam
    #[clap(short, long, default_value = "backplan.yaml")]
    pub jobfile: String,

    /// Interval between full backups (days)
    #[clap(short, long)]
    pub interval: Option<u8>, // TODO: handle errors of invalid input better?

    /// Force full backup
    #[clap(short, long)]
    pub full: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

#[derive(Debug, Deserialize)]
pub struct Backplan {
    back_paths: Vec<String>,
    exclude_paths: Vec<String>,
}

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    let back_plan = fs::read_to_string(args.jobfile)?;
    let back_plan: Backplan = serde_yaml::from_str(&back_plan)?;
    println!("{:?}", back_plan);
    // for filename in args.files {
    //     let file_contents = fs::read_to_string(&filename)?;
    //     let matching = if args.case {
    //         search(&args.query, &file_contents)
    //     } else {
    //         search_case_insensitive(&args.query, &file_contents)
    //     };

    //     if matching.len() != 0 {
    //         println!("=== In file {}:", filename);
    //     }
    //     for line in matching {
    //         println!("{}", line);
    //     }
    // }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        
    }
}