use crate::AOCResult;
use itertools::Itertools;


pub fn solution_1(input: String) -> AOCResult<u32> {
    let mut sum = 0;
    for rucksack in input.split("\n") {
        let length = rucksack.len();
        let (compartment_1, compartment_2) = rucksack.split_at(length/2);
        let mut misplaced = '∆';
        for letter in compartment_1.chars() {
            if compartment_2.contains(letter) {misplaced = letter; break;}
        }
        sum += to_priority(misplaced);

    }
    Ok(sum)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    let mut sum = 0;
    let mut groups = input.split("\n");
    while let Some((elf1, elf2, elf3)) = groups.next_tuple() {
        let mut misplaced = '∆';
        for letter in elf1.chars() {
            if elf2.contains(letter) &&
                elf3.contains(letter) {misplaced = letter; break;}
        }
        sum += to_priority(misplaced);
    }
    Ok(sum)
}

fn to_priority(letter: char) -> u32 {
    let ascii = letter as u32;
    let ordered = if ascii > 90 {ascii - 58} else {ascii};
    ordered - 38
}