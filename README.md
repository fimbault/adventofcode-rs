# adventofcode-rs
Advent of code solutions implemented in rust.
Credits for the challenges go to [adventofcode](https://adventofcode.com/), who do a fantastic job.

Notes:
* you should try by yourselves, before looking at the solutions. 
* there's not a single way to solve those challenges, maybe you'll find better ways. I personnally use it mostly as a teaching tool

## use as a template
See [tips](ARCHITECTURE.md)

## 2020

See the inputs directory for more description on the challenges.

### day 1: Report Repair
* parse the file (one value per line), thanks to iterators
* apply a basic algorithm to find the answer
* my solution shows how to deal with different integer types (u16 and u32)

### day 2: Password Philosophy
* use a regex (btw, https://regex101.com is a useful resource) to parse the input lines
* more iterators to check rules

### day 3: Toboggan Trajectory
* more filtering with closures within iterators

### day 4: Passport Processing
* this exercice is similar to JWT claims processing
* more work on the line parser, and then on the rules

### day 5: Binary Boarding
* convert into binary numbers


