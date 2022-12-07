using rust and nim for aoc this year. solutions will only use built-in libs. implementations may slightly differ. i am mostly collecting this data because i think it's neat.

# Deck the halls with boughs of benchmark
rust: `cargo run --release [day/s]`

nim: `nim c -r main.nim [day] -d:danger --gc:arc` (additional flags used for maximum perf)

| Day   | Rust   | Nim   | 
|---|---|---|
| 1   | 0.0451 ms  | 0.1861ms   | 
| 2   | 1.3260 ms  | 0.3668ms   |
| 3   | 0.4499 ms  | 4.3434ms   |
| 4   | 0.1567 ms  | n/a        |
| 5   | 0.1016 ms  | n/a        |
| 6   | 0.3461 ms  | n/a        |

does not include compilation or arg parsing. includes input file read time.
