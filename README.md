# adventofcode-rs

Advent of code solutions implemented in rust.
Credits for the challenges go to [adventofcode](https://adventofcode.com/), who do a fantastic job.

Notes:

- you should try by yourselves, before looking at the solutions.
- there's not a single way to solve those challenges, maybe you'll find better ways. I personnally use it mostly as a teaching tool

## use as a template

See [tips](ARCHITECTURE.md)

Commands:

- `cargo run` will build the solutions and run the main function
- `cargo test` will launch the tests defined in the project

## 2020

See the inputs directory for more description on the challenges.

### day 1: Report Repair

- parse the file (one value per line), thanks to iterators
- apply a basic algorithm to find the answer
- my solution shows how to deal with different integer types (u16 and u32)

### day 2: Password Philosophy

- use a regex (btw, https://regex101.com is a useful resource) to parse the input lines
- more iterators to check rules

### day 3: Toboggan Trajectory

- more filtering with closures within iterators

### day 4: Passport Processing

- this exercice is similar to JWT claims processing
- more work on the line parser, and then on the rules

### day 5: Binary Boarding

- convert into binary numbers

### day 6: Custom Customs

- more functional programming on iterators (maps, folds)

### day 7: Handy Haversacks

- parse the file and process the data using hashmap and hashset
- the solution uses lifetimes as we define structs that contain references
- we also implement tests, and use derive macros for eq/partialeq
- part 2 left as an exercice for the reader

### day 8: Handheld Halting

- map strings to enums (using try_from)
- implement a basic state machine (computer instructions)
- part 2 left as an exercice to the reader

### day 9: Encoding Error

- need to create slices of use only a part of the data (preamble vs remaining)
- here we use crate itertools to simplify the is_sum function
- part 2 left as an exercice to the reader

### day 10: Adapter Array

- basic operations with vectors (sort, push, insert, filter on a value and count)

### day 11: Seating System

- simple conversion from char to enum (from) - we don't use tryfrom as we expect no error
- hashmap to represent the layout
