using rust and nim for aoc this year. solutions will only use built-in libs. implementations may slightly differ. i am mostly collecting this data because i think it's neat.

# Deck the halls with boughs of benchmark
rust: vanilla `cargo run`

nim: using `-d:danger --gc:arc` for maximum perf 

| Day   | Rust   | Nim   | 
|---|---|---|
| 1   | 0.0451 ms  | 0.1861ms   | 
| 2   | 1.3260 ms  | 0.3668ms   |

does not include compilation or arg parsing. includes input file read time.
