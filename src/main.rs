mod death_cause;
mod parser;
mod report;

use std::fs::File;

use memmapix::MmapOptions;
use parser::Parser;

fn main() -> eyre::Result<()> {
    let file = File::open("./q3.log")?;
    let file_contents = unsafe { MmapOptions::new().map_copy_read_only(&file) }.unwrap();

    let reports = Parser::parse(&file_contents);

    for report in reports {
        println!("{report}");
    }

    Ok(())
}
