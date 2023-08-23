use std::collections::HashMap;

use self::log_kind::LogKind;
use crate::{
    death_cause::DeathCauseDb,
    errors::Error,
    parser::combinators::{consume_rest_of_line, kill, Assassin},
    report::Report,
};

mod combinators;
mod log_kind;

#[derive(Debug, Default)]
pub struct Parser<'buf> {
    game: u16,
    total_kills: u16,
    kills: HashMap<&'buf [u8], isize>,
    means: DeathCauseDb,
}

impl<'file> Parser<'file> {
    pub fn parse(buf: &'file [u8]) -> Result<Vec<Report<'file>>, (Error, Vec<Report<'file>>)> {
        let mut parser = Parser::default();

        parser.parse_lines(buf)
    }

    fn parse_lines(
        &mut self,
        mut buf: &'file [u8],
    ) -> Result<Vec<Report<'file>>, (Error, Vec<Report<'file>>)> {
        use nom::Parser;
        let mut reports = vec![];

        loop {
            let Ok((rest, (_time, logkind))) = (
                combinators::timestamp::<nom::error::Error<_>>,
                combinators::log_kind,
            )
                .parse(buf)
            else {
                return Err((Error::Parsing(), reports));
            };

            match logkind {
                LogKind::Kill => {
                    let Ok((_, info)) = kill::<nom::error::Error<_>>(rest) else {
                        return Err((Error::Parsing(), reports));
                    };

                    self.total_kills += 1;
                    match info.assassin {
                        Assassin::World => {
                            *self.kills.entry(info.victim).or_insert(0) -= 1;
                        }
                        Assassin::Person(person) => {
                            *self.kills.entry(person).or_insert(0) += 1;
                        }
                    }
                    self.means.inc_death(info.mean);
                }
                LogKind::ShutdownGame => {
                    reports.push(self.snapshot_report());
                }
                _ => {
                    // ignore
                }
            };

            if rest.is_empty() {
                break Ok(reports);
            }

            let Ok((newline, _)) = consume_rest_of_line::<nom::error::Error<_>>(rest) else {
                return Err((Error::Parsing(), reports));
            };

            buf = newline;
        }
    }

    fn snapshot_report(&mut self) -> Report<'file> {
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
