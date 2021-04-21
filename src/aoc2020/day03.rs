#![allow(dead_code)]
use crate::puzzle::get_puzzle;

pub fn challenge() {
    let input = get_puzzle(2020, 3).unwrap();

    println!("day 3: num trees = {}", count_trees(&input, 3, 1));

    let a = count_trees(&input, 1, 1);
    let b = count_trees(&input, 3, 1);
    let c = count_trees(&input, 5, 1);
    let d = count_trees(&input, 7, 1);
    let e = count_trees(&input, 1, 2);

    println!("day 3 - part 2: num trees = {}", a*b*c*d*e);

}


fn count_trees(forest: &str, r_step: usize, d_step: usize) -> usize {
    forest
        .lines()
        .step_by(d_step)
        .enumerate()
        .filter(|(i, x)| x.chars().nth(i * r_step % x.len()) == Some('#'))
        .count()
}

