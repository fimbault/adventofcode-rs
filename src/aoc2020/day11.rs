use crate::puzzle::get_puzzle;

pub fn challenge() {
    // get a vector filled with puzzle numbers
    let input = get_puzzle(2020, 11).unwrap();

    // read the seat layout
    let mut layout: Vec<Vec<Seat>> = vec![];
    input.lines().for_each(|line| {
        layout.push(decode(line));
    });

    // let people enter

    println!("day 11 (part 2): not implemented");
}

// each seat has a layout
#[derive(Clone, Copy)]
struct Seat {
    status: SeatStatus,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SeatStatus {
    None,     // .
    Empty,    // L
    Occupied, // #
}

impl From<char> for SeatStatus {
    fn from(value: char) -> Self {
        match value {
            '.' => SeatStatus::None,
            'L' => SeatStatus::Empty,
            '#' => SeatStatus::Occupied,
            e => panic!("invalid seat status {}", e),
        }
    }
}

fn decode(line: &str) -> Vec<Seat> {
    // transform into enum values
    let row = line
        .chars()
        .map(|c| Seat {
            status: SeatStatus::from(c),
        })
        .collect::<Vec<Seat>>();

    row
}

// returns the number of changes
// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.
fn seat_people(layout: &Vec<Vec<Seat>>) -> usize {
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
