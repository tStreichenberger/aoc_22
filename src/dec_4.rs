use crate::AOCResult;
use itertools::{self, Itertools};

pub fn solution_1(input: String) -> AOCResult<u32> {
    let mut total = 0;
    for pair in input.split("\n") {
        let (elf1, elf2) = parse_elves(pair)?;
        if elf1.contains(elf2) || elf2.contains(elf1) {total += 1};
    }
    Ok(total)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    let mut total = 0;
    for pair in input.split("\n") {
        let (elf1, elf2) = parse_elves(pair)?;
        if elf1.overlaps(elf2) {total += 1};
    }
    Ok(total)
}

type Elf = (u32, u32);

fn parse_elves(pair: &str) -> AOCResult<(Elf, Elf)> {
    let (elf1, elf2) = pair.split(",").next_tuple().unwrap();

    let elf1: (&str, &str) = elf1.split("-").next_tuple().unwrap();
    let elf2: (&str, &str) = elf2.split("-").next_tuple().unwrap();

    let elf1: Elf = (elf1.0.parse()?, elf1.1.parse()?);
    let elf2: Elf = (elf2.0.parse()?, elf2.1.parse()?);
    Ok((elf1, elf2))
}

trait ElfExt {
    fn contains(&self, other: Elf) -> bool;
    fn overlaps(&self, other: Elf) -> bool;
}

impl ElfExt for Elf {
    fn contains(&self, other: Elf) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: Elf) -> bool {
        (self.0 >= other.0 && self.0 <= other.1) ||
        (self.1 >= other.0 && self.1 <= other.1) || 
        self.contains(other)
    }
}