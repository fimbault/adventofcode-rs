use crate::puzzle::get_puzzle;

// use itertools to get tuple support
use itertools::Itertools;

pub fn challenge() {
    // get a vector filled with puzzle numbers
    let input = get_puzzle(2020, 9).unwrap();

    if let Some(result) = find_p1(&input, 25) {
        println!("day 9 (part 1): number = {}", result);
    }

    println!("day 9 (part 2): not implemented");
}

fn find_p1(input: &str, presize: usize) -> Option<usize> {
    // get all numbers
    let nums = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    // scan all search space (skip preamble)
    for (i, val) in nums.iter().enumerate().skip(presize) {
        // rolling check of size = presize : [0..25], [1..26], etc.
        if !is_sum(&nums[i - presize..i], *val) {
            return Some(*val);
        }
    }

    None
}

// is_sum returns true if we find 2 numbers in the preamble that sumup to the target
fn is_sum(preamble: &[usize], value: usize) -> bool {
    // tuple_combination is provided by itertools
    for pair in preamble.iter().tuple_combinations::<(_, _)>() {
        if pair.0 + pair.1 == value {
            return true;
        }
    }

    false
}

#[test]
fn day9_p1_preamble5_works() {
    let example = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    if let Some(result) = find_p1(&example, 5) {
        assert_eq!(result, 127);
    }
}
