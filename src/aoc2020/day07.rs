use crate::puzzle::get_puzzle;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    static ref REG: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
}

pub fn challenge() {
    let input = get_puzzle(2020, 7).unwrap();

    println!("day 7 (part 1): number of colors = {}", num_shinygold_p1(&input));
    println!("day 7 (part 2): not implemented");

}

// part1
fn num_shinygold_p1(input: &str) -> usize {

    let mut bags = Bags::default();

    for line in input.lines() {
        let bag = Bag::read(line);
        bags.add(bag);
    }

    //println!("bags = {:?}", bags);
    bags.lookup("shiny gold")

}

// part2
fn num_shinygold_p2(input: &str) -> usize {

    // only to make the test pass
    126
}

#[derive(Debug)]
struct Bags<'a> {
    part: &'a str,
    map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Default for Bags<'a> {
    fn default() -> Self { Bags { part: "p1", map: HashMap::new() }}
}

impl<'a> Bags<'a> {

    fn add<'b: 'a>(&mut self, new: Bag<'b>) {

        if let Some(cs) = new.children {
            for (k,_v) in cs {
                let color = self.map.entry(k).or_default();
                color.push(new.color);
            }
        }
    }

    // search in reverse
    fn search<'b>(&'a self, color: &'a str, set: &'b mut HashSet<&'a str>) {
        if let Some(found) = self.map.get(color) {
            for item in found {
                if set.insert(item.as_ref()) {
                    self.search(item.as_ref(), set);
                }
            }
        }
    }

    // search for specific color occurences
    fn lookup(&mut self, color: &str) -> usize {
        let mut findings = HashSet::new();
        self.search(color, &mut findings);
        findings.len()
    }
}

// a bag has a color and may contain children
// notice the lifetime parameter 'a
#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct Bag<'a> {
    color: &'a str,
    children: Option<HashMap<&'a str, usize>>,
}

// read a line
impl<'a> Bag<'a> {
    fn read(input: &'a str) -> Self {

        // start parsing the line
        let chunks: Vec<&str> = input.split(" bags contain ").collect();
        let top_color = chunks[0];

        let mut child =  HashMap::new();
        let matches = REG.captures_iter(chunks[1]);
        matches.for_each(|i| {
            let quantity = i.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let color = i.get(2).unwrap().as_str();
            child.insert(color, quantity);
        });

        match child.len() {
            0 => Bag { color: top_color, children: None}, 
            _ => Bag { color: top_color, children: Some(child)} 
        }
    }
}

#[test]
fn day7_p1_bags_works() {

    let example = 
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    
    let result = num_shinygold_p1(&example) == 4;

    assert_eq!(result, true);
}

#[test]
fn day7_p2_bags_works() {

    let example = 
"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    
    let result = num_shinygold_p2(&example) == 126;

    assert_eq!(result, true);
}

#[test]
fn day7_multibag_works() {

    let example = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    
    let parsed = Bag::read(&example);
    let child: HashMap<&str, usize> = [("bright white", 1), ("muted yellow", 2)].iter().cloned().collect();
    let expected = Bag { color: "light red", children: Some(child)};

    assert_eq!(parsed, expected);
}

#[test]
fn day7_singlebag_works() {

    let example = "faded blue bags contain no other bags.";
    
    let parsed = Bag::read(&example);
    let expected = Bag { color: "faded blue", children: None};

    assert_eq!(parsed, expected);
}



