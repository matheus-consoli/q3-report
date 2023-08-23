mod parser;
mod report;
mod death_cause;

use std::{fs::File, io::BufReader};

use parser::Parser;

fn main() -> eyre::Result<()> {
    let file = File::open("./q3.log")?;
    let contents = BufReader::new(file);

    let reports = Parser::parse_file(contents);

    for report in reports {
        println!("{report}");
    }

    Ok(())
}
