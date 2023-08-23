use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_until},
        take_while,
    },
    character::{complete::multispace1, digit1, is_newline, multispace0},
    combinator::value,
    error::ParseError,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use super::log_kind::LogKind;
use crate::death_cause::DeathCause;

/// Parse the timestamp from a log line.
///
/// Timestamp have the form `MM:ss`.
///
/// ```text
///  15:00  <rest>
/// ```
pub fn timestamp<'line, E: ParseError<&'line str>>(
    input: &'line str,
) -> IResult<&'_ str, (&'_ str, &'_ str), E> {
    separated_pair(preceded(multispace0(), digit1()), tag(":"), digit1()).parse(input)
}

/// Parse the type of the log line
///
/// See [`LogKind`] for all supported types.
///
/// ```text
/// <timestamp>: <log kind>
/// ```
pub fn log_kind<'line, E: ParseError<&'line str>>(
    input: &'line str,
) -> IResult<&'_ str, LogKind, E> {
    preceded(
        multispace0(),
        alt((
            value(LogKind::ClientBegin, tag("ClientBegin:")),
            value(LogKind::ClientConnect, tag("ClientConnect:")),
            value(LogKind::ClientDisconnect, tag("ClientDisconnect:")),
            value(
                LogKind::ClientUserinfoChanged,
                tag("ClientUserinfoChanged:"),
            ),
            value(LogKind::InitGame, tag("InitGame:")),
            value(LogKind::Item, tag("Item:")),
            value(LogKind::Kill, tag("Kill:")),
            value(LogKind::Say, tag("say:")),
            value(LogKind::Score, tag("score:")),
            value(LogKind::ShutdownGame, tag("ShutdownGame:")),
            value(LogKind::Dashline, many1(tag("-"))),
            value(LogKind::CtfScore, tag("red:")),
            value(LogKind::Exit, tag("Exit:")),
        )),
    )
    .parse(input)
}

/// Parse the content of a "Kill" log line
///
/// " 3 4 6: player1 killed Player 2 by MOD_ROCKET"
pub fn kill<'line, E: ParseError<&'line str>>(input: &'line str) -> IResult<&'_ str, KillInfo, E> {
    // "[ 3 4 6: ]player1 killed Player 2 by MOD_ROCKET"
    let (rest, _) = (take_until(":"), tag(":"), multispace1).parse(input)?;

    // "[player1] killed Player 2 by MOD_ROCKET"
    let (rest, assassin) = take_until(" killed")(rest).map(|(rest, assassin)| match assassin {
        "<world>" => (rest, Assassin::World),
        otherwise => (rest, Assassin::Person(otherwise)),
    })?;

    // "killed [Player 2] by MOD_ROCKET"
    let (rest, (_, victim)) = (tag(" killed "), take_until(" by")).parse(rest)?;
    // "by [MOD_ROCKET]"
    let (mean, _) = tag(" by ")(rest)?;
    let mean = DeathCause::from_str(mean).unwrap();

    let kill = KillInfo {
        assassin,
        victim,
        mean,
    };
    Ok(("", kill))
}

#[derive(Debug, PartialEq, Eq)]
pub struct KillInfo<'line> {
    pub assassin: Assassin<'line>,
    pub victim: &'line str,
    pub mean: DeathCause,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Assassin<'line> {
    World,
    Person(&'line str),
}

fn consume_rest_of_line<'line, E: ParseError<&'line [u8]>>(
    input: &'line [u8],
) -> IResult<&'_ [u8], &'_ [u8], E> {
    let (rest, (before, _)) = ((take_while(|c| !is_newline(c))), tag("\n")).parse(input)?;
    Ok((rest, before))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::VerboseError as E;

    #[test]
    fn test_parse_timestamp() {
        let parsed = timestamp::<E<_>>("  20:34 ClientConnect: 2");

        assert_eq!(parsed, Ok((" ClientConnect: 2", ("20", "34"))));
    }

    #[test]
    fn test_parse_log_kind() {
        let parsed = log_kind::<E<_>>(" Exit: Timelimit hit.");
        assert_eq!(parsed, Ok((" Timelimit hit.", LogKind::Exit)));
    }

    #[test]
    fn test_parse_dashline() {
        let parsed =
            log_kind::<E<_>>(" ------------------------------------------------------------");
        assert_eq!(parsed, Ok(("", LogKind::Dashline)));
    }

    #[test]
    fn test_parse_full_dashline() {
        let parsed = (timestamp::<E<_>>, log_kind)
            .parse(" 0:00 ------------------------------------------------------------\n");
        assert_eq!(parsed, Ok(("\n", (("0", "00"), LogKind::Dashline))));
    }

    #[test]
    fn test_parse_kill() {
        let parsed = kill::<E<_>>(" 3 4 6: player1 killed Player 2 by MOD_ROCKET");
        let exp_kill = KillInfo {
            assassin: Assassin::Person("player1"),
            victim: "Player 2",
            mean: DeathCause::Rocket,
        };

        assert_eq!(parsed, Ok(("", exp_kill)));
    }

    #[test]
    fn test_eat_line() {
        let parsed = consume_rest_of_line::<E<_>>(b"aaabbb\nnewline");

        assert_eq!(parsed, Ok(("newline".as_bytes(), "aaabbb".as_bytes())))
    }
}
