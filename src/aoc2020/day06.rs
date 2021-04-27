#![allow(dead_code)]
use crate::puzzle::get_puzzle;
use std::collections::HashSet;

// The Iterator type contains basic methods such as map(), filter() and fold(), among many other highly useful methods. 
// Maps apply a function to each element in a vector and return the result of each iteration to the next function.
// Filters works similarly only that it will only return elements that meet a certain criteria. 
// Folds will apply a function whose purpose is to accumulate all the elements in the vector into a single value. 
// The collect() function is used to return a new Vec of values.
// See https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter02.html

pub fn challenge() {
    let input = get_puzzle(2020, 6).unwrap();

    println!("day 6: number of distinct yes = {}", distinct_yes(&input));
    println!("day 6: number of everyone yes = {}", everyone_yes(&input));
}

fn distinct_yes(input: &str) -> usize {

    input
        .split("\n\n")   // split into a vec of responses
        .map(|line| {    // map each line in a response
            line.chars().filter(|c| 
                c.is_ascii_alphabetic())      // remove end of line (\n)
                .collect::<HashSet<char>>()   // for each remaining caracter
                .len()                        // get the length
        })
        .sum::<usize>()                       // sum for all lines in a response
}

fn everyone_yes(input: &str) -> usize {
    
    input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|c| c.chars().collect::<HashSet<char>>())
                .fold(('a'..='z').collect::<HashSet<char>>(), |acc, c| {
                    acc.intersection(&c).cloned().collect()
                })
                .len()
        })
        .sum::<usize>()

}


#[test]
fn day6_single1_works() {

    let example = 
"abcx
abcy
abcz";
    
    let result = distinct_yes(&example) == 6; 

    assert_eq!(result, true);
}

#[test]
fn day6_multi1_works() {

    let example = 
"abc

a
b
c

ab
ac

a
a
a
a

b";
    
    let result = distinct_yes(&example) == 11; 

    assert_eq!(result, true);
}

#[test]
fn day6_multi2_works() {

    let example = 
"abc

a
b
c

ab
ac

a
a
a
a

b";
    
    let result = everyone_yes(&example) == 6; 

    assert_eq!(result, true);
}