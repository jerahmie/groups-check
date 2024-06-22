use std::fs;
use std::io::{BufRead, BufReader, Error};
use clap::{Parser};
use libgroupscheck::{read_groups};
//use sscanf::sscanf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    // path to group file
    #[arg(required = true)]
    file_path: String,
    // reference group file
    // #[arg(required = true)]
    // ref_file: String,
    // optionally fix the file
    #[clap(long, short, action)]
    fix: bool,
}

fn main() -> Result<(), Error>{
    let args = Args::parse();
    let req_groups = read_groups(req_groups);&args.file_path); 
    
    println!("Fix group file? {:?}\n", args.fix);
    
    Ok(())
}
