use clap::{Parser, Error};
use libgroupscheck::{read_groups};

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
    let req_groups = read_groups(&args.file_path).unwrap(); 
    for grp in req_groups {
        println!("{:?}\n", grp[0]);
    }

    if args.fix {
        println!("Attempting to fix group file");
    }
    
    Ok(())
}
