use std::{collections::HashMap, io::BufRead, ops::ControlFlow};

use nom::error::ParseError;

use crate::{
    death_cause::DeathCauseDb,
    parser::combinators::{kill, Assassin},
    report::Report,
};

use self::log_kind::LogKind;

mod combinators;
mod log_kind;

#[derive(Debug, Default)]
pub struct Parser {
    game: u16,
    total_kills: u16,
    // TODO: use &str
    kills: HashMap<String, isize>,
    means: DeathCauseDb,
}

impl Parser {
    pub fn parse_file<A: BufRead>(mut content: A) -> Vec<Report> {
        let mut reports = Vec::new();
        let mut parser = Parser::default();
        let mut line = String::new();
        while let Ok(n) = content.read_line(&mut line) {
            if n == 0 {
                break;
            }
            match parser.parse_line::<nom::error::Error<_>>(&line) {
                Ok(ControlFlow::Break(_)) => {
                    reports.push(parser.snapshot_report());
                }
                Ok(ControlFlow::Continue(_)) => {}
                Err(er) => {
                    eprintln!("{er:?}");
                    std::process::exit(-1);
                }
            }
            line.clear();
        }
        reports
    }

    fn parse_line<'line, E: ParseError<&'line str> + std::fmt::Debug>(
        &mut self,
        line: &'line str,
    ) -> eyre::Result<ControlFlow<()>> {
        use nom::Parser;
        let (rest, (_timestamp, logkind)) = (combinators::timestamp::<E>, combinators::log_kind)
            .parse(line)
            .map_err(|e| eyre::eyre!("failed to parse line:\n{line}\n{e}"))?;

        match logkind {
            LogKind::Kill => {
                let (_, info) = kill::<E>(rest).unwrap();
                self.total_kills += 1;
                match info.assassin {
                    Assassin::World => {
                        *self.kills.entry(info.victim.to_string()).or_insert(0) -= 1;
                    }
                    Assassin::Person(person) => {
                        *self.kills.entry(person.to_string()).or_insert(0) += 1;
                    }
                }
                self.means.inc_death(info.mean);
            }
            LogKind::ShutdownGame => return Ok(ControlFlow::Break(())),
            _ => {
                // ignore
            }
        };
        Ok(ControlFlow::Continue(()))
    }

    fn snapshot_report(&mut self) -> Report {
        let report = Report {
            game_number: self.game,
            total_kills: self.total_kills,
            kills: self.kills.clone(),
            means: self.means.clone(),
        };

        self.game += 1;
        self.total_kills = 0;
        self.kills.clear();
        self.means = Default::default();

        report
    }
}

#[cfg(test)]
mod test {}
