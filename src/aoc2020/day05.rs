#![allow(dead_code)]
use crate::puzzle::get_puzzle;

pub fn challenge() {
    let input = get_puzzle(2020, 5).unwrap();

    let mut seats: Vec<u32> = vec![];
    input
        .lines()
        .for_each(|line| { 
            seats.push(decode_seat(&line)) }
        );

    let seatid_max = seats.iter().max_by(|a, b| a.partial_cmp(b).unwrap()); 
    let seatid_min = seats.iter().min_by(|a, b| a.partial_cmp(b).unwrap());

    println!("day 5: min seatID = {:?}, max seatID = {:?}", seatid_min, seatid_max);

    println!("day 5: my seat is at {}", find_empty_seat(&mut seats));

}

fn decode_seat(value: &str) -> u32 {

    // transform into binary string
    let bin: String = value.chars()
    .map(|x| match x { 
        'B' => '1',
        'F' => '0',
        'R' => '1',
        'L' => '0',
        _ => unreachable!(),
    }).collect();

    // now convert from binary to decimal
    let intval = u32::from_str_radix(&bin, 2).unwrap();

    return intval;

}

fn find_empty_seat(seats: &mut Vec<u32>) -> u32 {

    let mut my_seat = 0;
    for s in *seats.first().unwrap()..*seats.last().unwrap()
    {
        if !seats.contains(&s) { my_seat = s; }
    } 

    my_seat
}

#[test]
fn day5_works() {
    let result = decode_seat("BFFFBBFRRR") == 567 &&
    decode_seat("FFFBBBFRRR") == 119 &&
    decode_seat("BBFFBBFRLL") == 820; 

    assert_eq!(result, true);
}
