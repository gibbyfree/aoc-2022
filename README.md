using rust and nim for aoc this year. each solution will follow the same general implementation, and only use built-in libs.

# Deck the halls with boughs of benchmark
rust: vanilla `cargo run`

nim: using `-d:danger --gc:arc` for maximum perf 

| Day   | Rust   | Nim   | 
|---|---|---|
| 1   | 0.0451 ms  | 0.1861ms   | 

does not include compilation or arg parsing. includes input file read time.
