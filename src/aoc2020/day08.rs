use crate::puzzle::get_puzzle;

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    static ref REG: Regex = Regex::new(r"^(\w+) ([-+]\d+)").unwrap();
}

pub fn challenge() {
    // get a vector filled with puzzle numbers
    let input = get_puzzle(2020, 8).unwrap();

    println!("day 8 (part 1): acc = {}", execute(&input));
    println!("day 8 (part 2): not implemented");
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Opcode {
    Nop,
    Acc,
    Jmp,
}

const ACC_OPCODE: &str = "acc";
const JMP_OPCODE: &str = "jmp";
const NOP_OPCODE: &str = "nop";

impl Debug for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Nop => write!(f, "{}", NOP_OPCODE),
            Opcode::Acc => write!(f, "{}", ACC_OPCODE),
            Opcode::Jmp => write!(f, "{}", JMP_OPCODE),
        }
    }
}

#[derive(Debug)]
struct InvalidOpcode(String);

impl<'a> TryFrom<&'a str> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            NOP_OPCODE => Ok(Opcode::Nop),
            ACC_OPCODE => Ok(Opcode::Acc),
            JMP_OPCODE => Ok(Opcode::Jmp),
            _ => Err(InvalidOpcode(value.to_owned())),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Operation {
    code: Opcode,
    val: isize,
}

struct State<'a> {
    pos: usize,
    acc: isize,
    history: HashSet<usize>,
    ops: &'a Vec<Operation>,
}

impl<'a> State<'a> {
    fn new(instructions: &'a Vec<Operation>) -> Self {
        Self {
            pos: 0,
            acc: 0,
            history: HashSet::new(),
            ops: instructions,
        }
    }

    fn next(&mut self) {
        // update history
        self.history.insert(self.pos);

        let next_op = self.ops[self.pos];

        match next_op.code {
            Opcode::Nop => self.pos += 1,
            Opcode::Acc => {
                self.acc += next_op.val;
                self.pos += 1;
            }
            Opcode::Jmp => self.pos = (self.pos as isize + next_op.val) as usize,
        }
    }
}

fn execute(input: &str) -> isize {
    // start by reading the file
    let instructions = input
        .lines()
        .map(|line| {
            let matches = REG.captures(line).unwrap();
            let label = matches.get(1).unwrap().as_str();
            let _code = Opcode::try_from(label).unwrap();
            let _val = matches.get(2).unwrap().as_str().parse::<isize>().unwrap();
            Operation {
                code: _code,
                val: _val,
            }
        })
        .collect::<Vec<Operation>>();

    let mut state = State::new(&instructions);
    while state.pos < state.ops.len() && !state.history.contains(&state.pos) {
        state.next();
    }

    state.acc
}

#[test]
fn day8_p1_acc_works() {
    let example = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    let result = execute(&example) == 5;

    assert_eq!(result, true);
}
