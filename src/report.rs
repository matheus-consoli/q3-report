use std::{
    collections::HashMap,
    fmt::{self, Write},
};

use crate::death_cause::DeathCauseDb;

#[derive(Debug)]
pub struct Report {
    pub game_number: u16,
    pub total_kills: u16,
    pub kills: HashMap<String, isize>,
    pub means: DeathCauseDb,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = ' ';
        let mut buf = String::new();

        writeln!(&mut buf, "\"game_{}\": {{", self.game_number)?;

        writeln!(&mut buf, "{space:<2}\"total_kills\": {},", self.total_kills)?;

        let players = self.kills.keys();
        writeln!(&mut buf, "{space:<2}\"players\": {players:?},")?;
        write!(&mut buf, "{space:<2}\"kills\": {{")?;

        let mut kills = self.kills.iter();
        if let Some((player, kill_count)) = kills.next() {
            // handle the first player differently to format `,` appropriately
            write!(&mut buf, "\n{space:<4}\"{player}\": {kill_count}")?;
            for (player, kill_count) in kills {
                write!(&mut buf, ",\n{space:<4}\"{player}\": {kill_count}")?;
            }
            writeln!(&mut buf)?;
        }

        writeln!(&mut buf, "{space:<2}}}")?;

        write!(&mut buf, "{space:<2}\"kills_by_means\": {{")?;
        let mut means = self.means.counted();
        if let Some((count, mean)) = means.next() {
            write!(&mut buf, "\n{space:<4}\"{}\": {count}", mean.as_str())?;
            for (count, mean) in means {
                write!(&mut buf, ",\n{space:<4}\"{}\": {count}", mean.as_str())?;
            }
            writeln!(&mut buf)?;
        }
        writeln!(&mut buf, "{space:<2}}}")?;

        writeln!(&mut buf, "}}")?;

        write!(f, "{buf}")
    }
}
