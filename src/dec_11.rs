use crate::{
    AOCError,
    AOCResult,
    CustomError,
};
use std::{
    sync::RwLock,
    collections::{
        BinaryHeap,
        VecDeque,
    },
    str::FromStr,
};
use itertools::Itertools;

const MONKEY_MODULO: u32 = 19 * 7 * 17 * 13 * 11 * 2 * 5 * 3;

pub fn solution_1(input: String) -> AOCResult<u32> {
    // parse monkeys
    let mut monkeys = Vec::new();
    for monkey_str in input.split("\n\n") {
        let monkey: Monkey = monkey_str.parse()?;
        monkeys.push(monkey);
    }

    // do 20 rounds
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            while let Some((throw_to, item)) = monkey.inspect_next_1() {
                monkeys[throw_to].catch(item);
            }
        }
    }

    // find two highest throwing monkeys
    let mut scores = BinaryHeap::new();
    for monkey in monkeys.iter() {
        scores.push(monkey.times_inspected());
    }
    let highest = scores.pop().unwrap();
    let second_highest = scores.pop().unwrap();
    Ok(highest * second_highest)
}

pub fn solution_2(input: String) -> AOCResult<u64> {
    // parse monkeys
    let mut monkeys = Vec::new();
    for monkey_str in input.split("\n\n") {
        let monkey: Monkey = monkey_str.parse()?;
        monkeys.push(monkey);
    }

    // do 10,000 rounds
    for _ in 0..10_000 {
        for monkey in monkeys.iter() {
            while let Some((throw_to, item)) = monkey.inspect_next_2() {
                monkeys[throw_to].catch(item);
            }
        }
    }

    // find two highest throwing monkeys
    let mut scores = BinaryHeap::new();
    for monkey in monkeys.iter() {
        scores.push(monkey.times_inspected());
    }
    let highest = scores.pop().unwrap();
    let second_highest = scores.pop().unwrap();
    Ok(highest as u64 * second_highest as u64)
}

type Item = u32;

#[derive(Debug)]
struct Monkey {
    times_inspected: RwLock<u32>,
    items: RwLock<VecDeque<Item>>,
    op: Op,
    // for checking divisible by
    check: u32,
    // true or false monkey to throw to
    either: Either
}

impl Monkey {
    // returns the index of the monkey to throw to and the item to throw
    // returns none if monkey has no items
    fn inspect_next_1(&self) -> Option<(usize, Item)> {
        let item = match self.items.write().unwrap().pop_front() {
            Some(item) => item,
            None => return None
        };

        let new_item = self.op.exec(item) / 3;


        let next_monkey = match new_item % self.check == 0 {
            true => self.either.t,
            false => self.either.f,
        };

        // let old_maount = self.times_inspected.;
        *self.times_inspected.write().unwrap() += 1;

        Some((next_monkey, new_item))
    }

    fn inspect_next_2(&self) -> Option<(usize, Item)> {
        let item = match self.items.write().unwrap().pop_front() {
            Some(item) => item,
            None => return None
        };

        let new_item = self.op.exec(item);

        let next_monkey = match new_item % self.check == 0 {
            true => self.either.t,
            false => self.either.f,
        };

        // let old_maount = self.times_inspected.;
        *self.times_inspected.write().unwrap() += 1;

        Some((next_monkey, new_item))
    }

    fn catch(&self, item: Item) {
        self.items.write().unwrap().push_back(item);
    }

    fn times_inspected(&self) -> u32 {
        *self.times_inspected.read().unwrap()
    }
}

impl FromStr for Monkey {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let times_inspected = 0;
        let mut items = VecDeque::new();
        let mut lines = s.split("\n");

        let starting_items_str = lines.nth(1).unwrap();
        let rhs = starting_items_str.split(":").last().unwrap();
        for item_str in rhs.split(",") {
            let item = item_str.trim().parse()?;
            items.push_back(item);
        }

        let op_str = lines.next().unwrap();
        let op = op_str.parse()?;

        let check_str = lines.next().unwrap();
        let check = check_str.trim().split_whitespace().last().unwrap().parse()?;

        let either_string = lines.join("\n");
        let either = (&either_string).parse()?;

        Ok(Monkey {
            times_inspected: RwLock::new(times_inspected),
            items: RwLock::new(items),
            op,
            check,
            either,
        })
    }
}

#[derive(Debug)]
struct Either {
    t: usize,
    f: usize
}

impl FromStr for Either {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (true_str, false_str) = s.split("\n").next_tuple().unwrap();
        let t = true_str.trim().split_whitespace().last().unwrap().parse()?;
        let f = false_str.trim().split_whitespace().last().unwrap().parse()?;
        Ok(Either { t, f })
    }
}

#[derive(Debug)]
enum Op {
    // none means use old value
    Add(Option<u32>),
    Mul(Option<u32>)
}

impl Op {
    fn exec(&self, item: Item) -> Item {
        match self {
            Op::Add(opt_num) => {
                let num = opt_num.unwrap_or(item);
                item + num
            }
            Op::Mul(opt_num) => {
                let num = opt_num.unwrap_or(item);
                let large_num = num as u64;
                let large_item = item as u64;
                let large_modulo = MONKEY_MODULO as u64;
                ((large_item * large_num) % large_modulo) as u32
            }
        }
    }
}

impl FromStr for Op {
    type Err = AOCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rhs = s.split("=").last().unwrap();
        let rhs_split: Vec<&str> = rhs.split_whitespace().collect();

        let num_res = rhs_split[2].parse::<u32>();
        let num_opt = match num_res {
            Ok(num) => Some(num),
            Err(_) => None
        };

        match rhs_split[1] {
            "+" => Ok(Op::Add(num_opt)),
            "*" => Ok(Op::Mul(num_opt)),
            _ => Err(CustomError("Tried to parse op that is not add or mul".to_string()).into())
        }
    }
}