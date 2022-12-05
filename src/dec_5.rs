use crate::AOCResult;
use itertools::Itertools;

pub fn solution_1(input: String) -> AOCResult<String> {
    let (crate_str, commands) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = parse_stacks(crate_str);
    for command_str in commands.split("\n") {
        let command = parse_command(command_str)?;
        stacks.execute_command_9000(command);
    }
    let mut tops = String::new();
    for stack in stacks.iter_mut() {
        tops.push(stack.pop().unwrap())
    }
    Ok(tops)
}

pub fn solution_2(input: String) -> AOCResult<String> {
    let (crate_str, commands) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = parse_stacks(crate_str);
    for command_str in commands.split("\n") {
        let command = parse_command(command_str)?;
        stacks.execute_command_9001(command);
    }
    let mut tops = String::new();
    for stack in stacks.iter_mut() {
        tops.push(stack.pop().unwrap())
    }
    Ok(tops)
}

type Crate = char;

type Stack = Vec<Crate>;

fn parse_stacks(crate_str: &str) -> Vec<Stack> {
    let mut lines = crate_str.split("\n").collect::<Vec<&str>>();
    let indicies = lines.pop().unwrap();
    let num_stacks = indicies.trim().split_whitespace().count();
    let mut stacks = Vec::new();
    for _ in 0..num_stacks {stacks.push(Stack::new())};
    while let Some(crates) = lines.pop() {
        for i in 0..num_stacks {
            let crt_index = i*4 + 1;
            let crt: Crate = crates.chars().nth(crt_index).unwrap();
            if crt != ' ' {
                stacks.get_mut(i).unwrap().push(crt);
            }
        }
    }
    stacks
}

type Command = (usize, usize, usize);

fn parse_command(command_str: &str) -> AOCResult<Command> {
    let mut split = command_str.split_whitespace();
    let amount: usize = split.nth(1).unwrap().parse()?;
    let from: usize = split.nth(1).unwrap().parse()?;
    let to: usize = split.nth(1).unwrap().parse()?;
    Ok((amount, from, to))
}

trait CrateMover9000 {
    fn execute_command_9000(&mut self, command: Command);
}

impl CrateMover9000 for Vec<Stack> {
    fn execute_command_9000(&mut self, command: Command) {
        for _ in 0..command.0 {
            let crt: Crate = self[command.1 - 1].pop().unwrap();
            self[command.2 - 1].push(crt);
        }
    }
}

trait CrateMover9001 {
    fn execute_command_9001(&mut self, command: Command);
}

impl CrateMover9001 for Vec<Stack> {
    fn execute_command_9001(&mut self, command: Command) {
        let mut picked_up = Stack::new();
        for _ in 0..command.0 {
            let crt: Crate = self[command.1 - 1].pop().unwrap();
            picked_up.push(crt);
        }
        for crt in picked_up.into_iter().rev() {
            self[command.2 - 1].push(crt);
        }
    }
}