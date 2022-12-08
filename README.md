# Advent of Code 2022

Solutions for [Advent of Code 2022](https://adventofcode.com/2022) using [Rust](https://www.rust-lang.org/).
This repository uses a template of [fspoettel](https://github.com/fspoettel/advent-of-code-rust) to make an extremely comfortable experience. Go give them some star!!

<!--- advent_readme_stars table --->

## Times

The following times were recorded in my local machine. Parsing time is usually considered out of the equation but I will probably update the code to add that measuring too.

|        | Part 1   | Part 2    |
|--------|----------|-----------|
| Day 01 | 8.03µs   | 22.28µs   | 
| Day 02 | 35.27µs  | 36.60µs   |
| Day 03 | 208.27µs | 207.92µs  |
| Day 04 | 2.86µs   | 1.89µs    |
| Day 05 | 23.96µs  | 64.81µs   |
| Day 06 | 14.74µs  | 29.75µs   |
| Day 07 | 37.72µs  | 32.90µs   |
| Day 08 | 31.50µs  | 225.73µs  |

Total running time for all days and parts: 0.67ms (at the moment)

## Some thoughs
* The template of *fspoettel* is awesome but I tweaked a bit to match my intentions, like parsing the input just once for both parts if it's possible.
* The ASCII inputs are awesome to work with bytes instead of strings and improve the performance times.
* `Rc<RefCell<T>>` combo really asks for macros and the use of `type` to stop those painfully long lines.
* I love to craft specialized data structures like I had to do on **Day 06**.
* I'm pretty proud of the decision to build a bigger array for the `Marker` data structure of **Day 06**. It allowed me to do the array advancement a lot better.
* The **Day 06** I decided to use traits and a bit of duplicated code. I just wanted to use arrays and that requires sizes known at compilation time. I also wanted to make some solution based on default trait implementations for dedicated structs.
* I was trying to make some weird things to solve **Day 07** instead of the logic approaches and that made me lose a lot of time. I was actually defeated the first day and had to left it to the next. It was easy with the obvious approach. 
* For **Day 08** I looked the size of the grid manually to use arrays. I see it as part of the optimization, sorry not sorry. To deal with the difference between the example and input I used the `[cfg()]` macro and it worked wonders. Macros were also a big part of this day.