mod death_cause;
mod errors;
mod parser;
mod report;

use std::fs::File;

use errors::Error;
use eyre::Context;
use memmapix::{Mmap, MmapOptions};
use parser::Parser;

fn main() -> eyre::Result<()> {
    simple_eyre::install()?;
    let filename = std::env::args().nth(1).ok_or(Error::NoInputFile)?;

    let file = File::open(&filename).map_err(|e| Error::FileNotFound(e, filename))?;

    let file_contents = read_file(&file);

    match Parser::parse(&file_contents) {
        Ok(reports) => {
            for report in reports {
                println!("{report}");
            }
            Ok(())
        }
        Err((error, reports)) => {
            for report in reports {
                eprintln!("{report}");
            }
            Err(error).wrap_err("failed to parse the entire file, dumped the successful results")
        }
    }
}

fn read_file(file: &File) -> Mmap {
    // Safety: using `mmap` in read-only mode is mostly safe.
    // The only concern is if the file is changed by some other process while being
    // read by this application.
    //
    // see: https://docs.rs/memmapix/0.7.3/memmapix/struct.MmapOptions.html#safety
    unsafe { MmapOptions::new().map_copy_read_only(file) }.unwrap()
}
