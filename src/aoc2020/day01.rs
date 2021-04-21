#![allow(dead_code)]
use crate::puzzle::get_puzzle;

pub fn challenge() {

    // get a vector filled with puzzle numbers
    let input = get_puzzle(2020, 1).unwrap();
    let mut nums = vec![];
    input
        .lines()
        .for_each(|line| nums.push(line.parse().unwrap()));

    // part 1 : search for target = 2020 = x+y
    let target: u16 = 2020;
    let(x,y) = search(&nums, target);
    let s1: u32 = u32::from(x) * u32::from(y);
    println!("day1 / part1: with x = {} and y = {}, x+y = 2020, x*y= {}", x, y, s1);

    // part 2 : search for target = 2020 = x+y+z
    let(x,y,z) = search_3(&nums, target);
    let s2: u32 = u32::from(x) * u32::from(y) * u32::from(z);
    println!("day1 / part2: with x = {} and y = {} and z = {}, x+y+z = 2020, x*y*z= {}", x, y, z, s2);
}

// part 1
fn search(nums: &Vec<u16>, target: u16) -> (u16, u16) {

    for i in nums.iter() {
        let j = target - *i;
        if nums.contains(&j) {
            return (*i, j);
        }
    }

    // we should have found something (if file is correct)
    unreachable!()
}

// part 2
fn search_3(nums: &Vec<u16>, target: u16) -> (u16, u16, u16) {
    
    for i in nums.iter() {
        for j in nums.iter() {
            // avoid negative k
            if target > *i + *j
            {
                let k = target - *i - *j;
                if nums.contains(&k) {
                    return (*i, *j, k);
                }
            }
        }
    }
    
    // we should have found something (if file is correct)
    unreachable!()
}