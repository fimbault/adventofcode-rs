#![allow(dead_code)]
use crate::puzzle::get_puzzle;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+)\s([a-z]+):\s+(\w+)").unwrap();
}

pub struct Policy {
    min: u8,
    max: u8, 
    symbol: char,
}

pub struct Pwd<'a> {
    policy: Policy,
    value: &'a str
}

pub fn challenge() {

    let input = get_puzzle(2020, 2).unwrap();
    let mut data: Vec<Pwd> = vec![];

    input.lines().for_each(|line| {
        if let Some(p) = custom_parser(line) {
            data.push(p);
        }
        
    });

    let mut count: u16 = 0;
    for i in data.iter() {
        match check_password(&i.value, &i.policy) {
            true => count +=1,
            false => (),
        }

    }

    let mut count2: u16 = 0;
    for i in data.iter() {
        match check_password_part2(&i.value, &i.policy) {
            true => count2 +=1,
            false => (),
        }

    }

    println!("day 2: {} valid passwords for policy1, {} valid passwords for policy2", count, count2);
}

fn custom_parser(line: &str) -> Option<Pwd> {

    if let Some(found) = REGEX.captures(line) {
        let val1: u8 = found.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let val2: u8 = found.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let val3: char = found.get(3).unwrap().as_str().parse::<char>().unwrap();
        let val4: &str = found.get(4).unwrap().as_str();
        return Some(Pwd{policy: Policy{min: val1, max: val2, symbol: val3}, value: val4});
    }
    else {
        return None;
    }
}

fn check_password(pwd: &str, policy: &Policy) -> bool {
    let n = pwd.chars().filter(|x| *x == policy.symbol).count() as u8;
    n >= policy.min && n <= policy.max
}

fn check_password_part2(pwd: &str, policy: &Policy) -> bool {
    let in_min = pwd
        .chars()
        .nth(policy.min as usize - 1)
        .filter(|x| *x == policy.symbol)
        .is_some();
    let in_max = pwd
        .chars()
        .nth(policy.max as usize - 1)
        .filter(|x| *x == policy.symbol)
        .is_some();
    in_min != in_max
}