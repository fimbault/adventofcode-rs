#![allow(dead_code)]
use crate::puzzle::get_puzzle;
use std::ops::RangeInclusive;

pub fn challenge() {
    let input = get_puzzle(2020, 4).unwrap();

    println!("day 4: {} valid passports", basic_check_fields(&input));

    println!("day 4: {} valid passports after applying complete rules", advanced_check_fields(&input));

}

fn basic_check_fields(input: &str) -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    
    // split by entry (separated by newlines) and then filter out
    input
        .split("\n\n")
        .filter(|entry| {fields.iter().fold(true, |acc, &x| acc && entry.contains(x))})
        .count()  
}

fn advanced_check_fields(input: &str) -> usize {
    
    input
        .split("\n\n")
        .filter(|entry| {validate(entry)})
        .count()  
}

fn validate(entry: &str) -> bool {
    entry
        .split_whitespace()
        .map(|i| {
            let mut s = i.split(":");
            let field = s.next().unwrap();
            let data = s.next().unwrap();
            (field, data)
        })
        .filter(|(f, d)| rules(f, d))
        .count()
        == 7
}

fn rules(field: &str, data: &str) -> bool {

    match field {
        "byr" if data.len() == 4 => is_in_range(data, 1920..=2002), 
        "iyr" if data.len() == 4 => is_in_range(data, 2010..=2020),
        "eyr" if data.len() == 4 => is_in_range(data, 2020..=2030),
        "hgt" if data.ends_with("cm") => is_in_range(data.split_at(data.len() - 2).0, 150..=193),
        "hgt" if data.ends_with("in") => is_in_range(data.split_at(data.len() - 2).0, 59..=76),
        "hcl" if data.len() == 7 && data.starts_with("#") => data.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        "ecl" => matches!(data, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" if data.len() == 9 => true,
        _ => false,
    }

}

fn is_in_range(data: &str, range: RangeInclusive<u16>) -> bool {
    data.parse::<u16>()
        .ok()
        .filter(|x| range.contains(x))
        .is_some()
}


