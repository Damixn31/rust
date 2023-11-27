mod args;
mod file;
mod list;
mod owners;
mod permissions;

mod options {
    pub mod filter;
    pub mod format;
    pub mod options;
    pub mod sort;
}

use std::error::Error;
use std::path::Path;

use args::Args;
use clap::Parser;

use options::filter::{all, default, dirs};
use options::format::{long, short};
use options::options::*;
use options::sort::{no_order, size, time};

use list::list;

pub fn get_options(args: &Args) -> (Filter, Sort, Format) {
    (
        if args.dirs {
            Filter::Dirs
        } else {
            if args.all {
                Filter::All
            } else {
                Filter::None
            }
        },
        if args.time {
            Sort::Time
        } else {
            if args.size {
                Sort::Size
            } else {
                Sort::None
            }
        },
        if args.long {
            Format::Long
        } else {
            Format::Short
        },
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let options = get_options(&args);

    let path = Path::new(&args.path);

    if path.is_dir() {
        println!(
            "{}",
            list(
                path,
                match options.0 {
                    Filter::None => default,
                    Filter::All => all,
                    Filter::Dirs => dirs,
                },
                match options.1 {
                    Sort::None => no_order,
                    Sort::Size => size,
                    Sort::Time => time,
                },
                match options.2 {
                    Format::Short => short,
                    Format::Long => long,
                }
            )?
        );
    } else {
        println!("{}", args.path);
    }

    Ok(())
}
