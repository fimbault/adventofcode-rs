use crate::puzzle::get_puzzle;

use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

pub fn challenge() {
    // get a vector filled with puzzle numbers
    let input = get_puzzle(2020, 11).unwrap();

    // read the seat layout
    let layout = Layout::new(&input);

    // let people enter

    println!("day 11 (part 2): not implemented");
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Clone)]
struct Area {
    area: HashMap<Position, SeatStatus>,
    x_len: usize,
    y_len: usize,
}

#[derive(Clone)]
struct Layout {
    area: Area,
    round: usize,
}

impl Layout {
    // create the layout
    fn new(input: &str) -> Self {
        let mut seats: HashMap<Position, SeatStatus> = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                seats.insert(
                    Position {
                        // we add 1 because when computing adjacent we don't want to get into negative
                        // also easier for visual checks as we start at 1 (and not 0)
                        row: i + 1,
                        column: j + 1,
                    },
                    SeatStatus::from(c),
                );
            }
        }

        Self {
            area: Area {
                area: seats.clone(),
                x_len: seats.keys().max_by_key(|&p| p.row).unwrap().row,
                y_len: seats.keys().max_by_key(|&p| p.column).unwrap().column,
            },
            round: 0,
        }
    }
}

// each seat has a layout
#[derive(Clone, Copy, Debug)]
struct Seat {
    status: SeatStatus,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SeatStatus {
    None,     // .
    Empty,    // L
    Occupied, // #
}

const NONE: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

impl Debug for SeatStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SeatStatus::None => write!(f, "{}", NONE),
            SeatStatus::Empty => write!(f, "{}", EMPTY),
            SeatStatus::Occupied => write!(f, "{}", OCCUPIED),
        }
    }
}

impl From<char> for SeatStatus {
    fn from(value: char) -> Self {
        match value {
            NONE => SeatStatus::None,
            EMPTY => SeatStatus::Empty,
            OCCUPIED => SeatStatus::Occupied,
            _ => unreachable!(),
        }
    }
}

// returns the number of changes
// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.
fn seat_people(layout: &Layout) -> usize {
    // need to find adjacent positions
    // apply the rules
    0
}

// returns number of occupied
fn compute_p1(input: &str) -> usize {
    // loop until there's no more change

    0
}

#[test]
fn day11_p1_works() {
    let mut example = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    let count = compute_p1(&mut example);

    assert_eq!(count, 37);
}
