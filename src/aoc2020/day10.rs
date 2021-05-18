use crate::puzzle::get_puzzle;

pub fn challenge() {
    // get a vector filled with puzzle numbers
    let input = get_puzzle(2020, 10).unwrap();

    let data = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    let counts = chain_p1(&data);
    println!("day 10 (part 1): value = {}", counts[0] * counts[1]);
    println!("day 10 (part 2): not implemented");
}

fn chain_p1(data: &Vec<usize>) -> [usize; 2] {
    let mut jolts = data.clone();

    // re-order
    jolts.sort();

    // start from 0 at the beginning
    jolts.insert(0, 0);

    // add a 3 jolt at the end
    jolts.push(jolts.iter().max().unwrap() + 3);

    // compute diff between 2 successive values
    // here done using windows but zip would also work
    // downside : windows copies the data
    let diffs = jolts
        .windows(2)
        .map(|t| t[1] - t[0])
        .collect::<Vec<usize>>();

    // return counts
    [
        diffs.iter().filter(|x| **x == 1).count(),
        diffs.iter().filter(|x| **x == 3).count(),
    ]
}

#[test]
fn day10_p1_example1_works() {
    let example: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let counts = chain_p1(&example);

    assert_eq!(counts[0], 7);
    assert_eq!(counts[1], 5);
}
