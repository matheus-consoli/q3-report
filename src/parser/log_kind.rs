/// Every possible log kind a log line may have.
// you can find all possible kinds with:
// `awk '{print $2}' logfile.log | sort | uniq`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogKind {
    ClientBegin,
    ClientConnect,
    ClientDisconnect,
    ClientUserinfoChanged,
    Exit,
    InitGame,
    Item,
    Kill,
    Say,
    Score,
    ShutdownGame,
    // `-------`
    Dashline,
    // red:N blue:M
    CtfScore,
}
