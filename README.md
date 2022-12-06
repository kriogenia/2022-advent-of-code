# Advent of Code 2022

Solutions for [Advent of Code 2022](https://adventofcode.com/2022) using [Rust](https://www.rust-lang.org/).
This repository uses a template of [fspoettel](https://github.com/fspoettel/advent-of-code-rust) to make an extremely comfortable experience. Go give them some star!!

<!--- advent_readme_stars table --->

## Times

The following times were recorded in my local machine. Parsing time is usually considered out of the equation.

|        | Part 1   | Part 2   |
|--------|----------|----------|
| Day 01 | 8.03µs   | 22.28µs  | 
| Day 02 | 35.27µs  | 36.60µs  |
| Day 03 | 208.27µs | 207.92µs |
| Day 04 | 2.86µs   | 1.89µs   |
| Day 05 | 23.96µs  | 64.81µs  |
| Day 06 | 14.74µs  | 29.75µs  |

Total running time for all days and parts: 0.66ms (at the moment)

## Some thoughs
* The template of *fspoettel* is awesome but I tweaked a bit to match my intentions, like parsing the input just once for both parts if it's possible.
* The ASCII inputs are awesome to work with bytes instead of strings and improve the performance times.
* I love to craft specialized data structures like I had to do on **Day 06**.
* I'm pretty proud of the decision to build a bigger array for the `Marker` data structure of **Day 06**. It allowed me to do the array advancement a lot better.
* The **Day 06** I decided to use traits and a bit of duplicated code. I just wanted to use arrays and that requires sizes known at compilation time. I also wanted to make some solution based on default trait implementations for dedicated structs.