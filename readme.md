# Q3-report

Parsing log files from Quake 3 Arena using Rust, mmap, and nom.

## Running

```shell
cargo run --release -- <file.log>
```

This repository have two log files, one that succeeds to parse, and one that fails.

When the parser fails, it will print the report for every game parsed until then.
