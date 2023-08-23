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

type Bytes<'file> = &'file [u8];

/// Parse the timestamp from a log line.
///
/// Timestamp have the form `MM:ss`.
///
/// ```text
///  15:00  <rest>
/// ```
pub fn timestamp<'file, E: ParseError<Bytes<'file>>>(
    input: Bytes<'file>,
) -> IResult<Bytes<'file>, (Bytes<'file>, Bytes<'file>), E> {
    separated_pair(preceded(multispace0(), digit1()), tag(":"), digit1()).parse(input)
}

/// Parse the type of the log line
///
/// See [`LogKind`] for all supported types.
///
/// ```text
/// <timestamp>: <log kind>
/// ```
pub fn log_kind<'file, E: ParseError<Bytes<'file>>>(
    input: Bytes<'file>,
) -> IResult<Bytes<'file>, LogKind, E> {
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
pub fn kill<'file, E: ParseError<Bytes<'file>>>(
    input: Bytes<'file>,
) -> IResult<Bytes<'file>, KillInfo, E> {
    // "[ 3 4 6: ]player1 killed Player 2 by MOD_ROCKET"
    let (rest, _) = (take_until(":"), tag(":"), multispace1).parse(input)?;

    // "[player1] killed Player 2 by MOD_ROCKET"
    let (rest, assassin) = take_until(" killed")(rest).map(|(rest, assassin)| match assassin {
        b"<world>" => (rest, Assassin::World),
        otherwise => (rest, Assassin::Person(otherwise)),
    })?;

    // "killed [Player 2] by MOD_ROCKET"
    let (rest, (_, victim)) = (tag(" killed "), take_until(" by")).parse(rest)?;
    // "by [MOD_ROCKET]"
    let (rest, (_, mean)) = (tag(" by "), take_while(|c| !is_newline(c))).parse(rest)?;
    let mean = std::str::from_utf8(mean).unwrap();
    let mean = DeathCause::from_str(mean).unwrap();

    let kill = KillInfo {
        assassin,
        victim,
        mean,
    };
    Ok((rest, kill))
}

#[derive(Debug, PartialEq, Eq)]
pub struct KillInfo<'file> {
    pub assassin: Assassin<'file>,
    pub victim: Bytes<'file>,
    pub mean: DeathCause,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Assassin<'file> {
    World,
    Person(Bytes<'file>),
}

pub fn consume_rest_of_line<'file, E: ParseError<Bytes<'file>>>(
    input: Bytes<'file>,
) -> IResult<Bytes<'file>, Bytes<'file>, E> {
    let (rest, (before, _)) = ((take_while(|c| !is_newline(c))), tag("\n")).parse(input)?;
    Ok((rest, before))
}

#[cfg(test)]
mod test {
    use nom::error::VerboseError as E;

    use super::*;

    #[test]
    fn test_parse_timestamp() {
        let parsed = timestamp::<E<_>>(b"  20:34 ClientConnect: 2");

        assert_eq!(
            parsed,
            Ok((
                " ClientConnect: 2".as_bytes(),
                ("20".as_bytes(), "34".as_bytes())
            ))
        );
    }

    #[test]
    fn test_parse_log_kind() {
        let parsed = log_kind::<E<_>>(b" Exit: Timelimit hit.");
        assert_eq!(parsed, Ok((" Timelimit hit.".as_bytes(), LogKind::Exit)));
    }

    #[test]
    fn test_parse_dashline() {
        let parsed =
            log_kind::<E<_>>(b" ------------------------------------------------------------");
        assert_eq!(parsed, Ok(("".as_bytes(), LogKind::Dashline)));
    }

    #[test]
    fn test_parse_full_dashline() {
        let parsed = (timestamp::<E<_>>, log_kind)
            .parse(b" 0:00 ------------------------------------------------------------\n");
        assert_eq!(
            parsed,
            Ok((
                "\n".as_bytes(),
                (("0".as_bytes(), "00".as_bytes()), LogKind::Dashline)
            ))
        );
    }

    #[test]
    fn test_parse_kill() {
        let parsed = kill::<E<_>>(b" 3 4 6: player1 killed Player 2 by MOD_ROCKET\nnextline");
        let exp_kill = KillInfo {
            assassin: Assassin::Person(b"player1"),
            victim: b"Player 2",
            mean: DeathCause::Rocket,
        };

        assert_eq!(parsed, Ok(("\nnextline".as_bytes(), exp_kill)));
    }

    #[test]
    fn test_eat_line() {
        let parsed = consume_rest_of_line::<E<_>>(b"");

        assert_eq!(parsed, Ok(("newline".as_bytes(), "aaabbb".as_bytes())))
    }
}
