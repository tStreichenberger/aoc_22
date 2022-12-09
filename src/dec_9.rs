use crate::AOCResult;
use std::collections::HashSet;

pub fn solution_1(input: String) -> AOCResult<usize> {
    let mut head: Pos = (0,0);
    let mut tail: Pos = (0,0);

    let mut unique_loc = HashSet::new();
    unique_loc.insert(tail);

    for line in input.split("\n") {
        let command = Command::parse(line);
        for _ in 0..command.amount {
            head.move_in(command.dir);
            tail.get_pulled_to(head);
            unique_loc.insert(tail);
        }
    }
    Ok(unique_loc.iter().count())
}

pub fn solution_2(input: String) -> AOCResult<usize> {
    let mut rope = Vec::new();
    for _ in 0..10 {rope.push((0,0))};

    let mut unique_loc = HashSet::new();
    unique_loc.insert(rope[9]);

    for line in input.split("\n") {
        let command = Command::parse(line);
        for _ in 0..command.amount {
            
            rope[0].move_in(command.dir);
            for i in 1..10 {
                let prev = rope[i-1];
                rope[i].get_pulled_to(prev);
            }
            unique_loc.insert(rope[9]);
        }
    }
    Ok(unique_loc.iter().count())
}

type Pos = (i32, i32);

trait PosExt {
    fn move_in(&mut self, dir: Pos);
    fn get_pulled_to(&mut self, head: Pos);
}

impl PosExt for Pos {
    fn move_in(&mut self, dir: Pos) {
        self.0 += dir.0;
        self.1 += dir.1;
    }

    fn get_pulled_to(&mut self, head: Pos) {
        let x_diff = head.0 - self.0;
        let y_diff = head.1 - self.1;
        if x_diff.abs() > 1 {
            self.0 += x_diff.signum(); 
            self.1 += y_diff.signum();
        } else if y_diff.abs() > 1 {
            self.0 += x_diff.signum();
            self.1 += y_diff.signum();
        }
    }
}

struct Command {
    dir: Pos,
    amount: u32
}

impl Command {
    fn parse(line: &str) -> Command {
        let (dir, amount) = line.split_at(1);
        let dir: Pos = match dir.trim() {
            "U" => (0,1),
            "D" => (0,-1),
            "L" => (-1,0),
            "R" => (1,0),
            err => panic!("Tried to Parse {err} as direction")
        };
        let amount = amount.trim().parse().unwrap();
        Command {
            dir,
            amount
        }
    }
}