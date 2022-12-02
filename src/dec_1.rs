use std::{
    collections::BinaryHeap,
    cmp::{
        max,
        Reverse
    },
    error::Error,
};

pub fn solution_1(input: String) -> Result<u64, Box<dyn Error>> {
    let elves = input.split("\n\n");

    let mut max_calories = 0;

    for elf in elves {
        let rations = elf.split("\n");
        let mut total_calories = 0;
        for food in rations {
            let calories: u64 = str::parse(food)?;
            total_calories += calories;
        }
        max_calories = max(max_calories, total_calories);
    }
    Ok(max_calories)
}


pub fn solution_2(input: String) -> Result<u64, Box<dyn Error>> {

    let elves = input.split("\n\n");

    let mut top_3  = Top3::new();
    for _ in 0..3 {top_3.push(Reverse(0))}

    for elf in elves {
        let rations = elf.split("\n");
        let mut total_calories = 0;
        for food in rations {
            let calories: u64 = str::parse(food)?;
            total_calories += calories;
        }
        top_3.replace_smallest(total_calories);
    }
    Ok(top_3.sum())
}

type Top3 = BinaryHeap::<Reverse<u64>>;

trait Top3Ext {
    fn replace_smallest(&mut self, calories: u64);

    fn sum(self) -> u64;
}

impl Top3Ext for Top3 {
    fn replace_smallest(&mut self, calories: u64) {
        if let Some(smallest) = self.peek() {
            if smallest.0 < calories {
                self.pop();
                self.push(Reverse(calories))
            }
        }
    }

    fn sum(self) -> u64 {
        let mut sum = 0;
        for calories in self.iter() {
            sum += calories.0;
        }
        sum
    }
}