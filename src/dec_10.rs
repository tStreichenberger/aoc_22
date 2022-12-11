use crate::{
    AOCError,
    AOCResult
};
use std::str::FromStr;

macro_rules! pop_next {
    ($commands:expr) => {
        match $commands.next() {
            Some(s) => s.parse()?,
            None => {break}
        }
    };
}

pub fn solution_1(input: String) -> AOCResult<i32> {
    let mut commands = input.split("\n");
    let mut curr_command = Command::NoOp;
    let mut cycle = 1;
    let mut x = 1;
    let mut total_signal = 0;
    loop {
        /* Start of Cycle */
        // start next op
        match curr_command {
            Command::BeginAdd(num) => curr_command = Command::Add(num),
            _ => curr_command = pop_next!(commands)
        }

        /* During Cycle */
        // update signal if correct cycle
        total_signal.check_cycle(cycle, x);

        /* End of Cycle */
        // execute add command
        if let Command::Add(num) = curr_command {
            x += num;
        }
        // increment cycle
        cycle += 1;
    }
    Ok(total_signal)
}

pub fn solution_2(input: String) -> AOCResult<String> {
    let mut commands = input.split("\n");
    let mut curr_command = Command::NoOp;
    let mut cycle = 1;
    let mut x = 1;
    let mut screen = String::new();
    loop {
        /* Start of Cycle */
        // start next op
        match curr_command {
            Command::BeginAdd(num) => curr_command = Command::Add(num),
            _ => curr_command = pop_next!(commands)
        }

        /* During Cycle */
        // draw pixel
        screen.draw_pixel(cycle, x);

        /* End of Cycle */
        // execute add command
        if let Command::Add(num) = curr_command {
            x += num;
        }
        // increment cycle
        cycle += 1;
    }
    Ok(screen)
}


trait SignalExt {
    fn check_cycle(&mut self, cycle: i32, x: i32);
}

impl SignalExt for i32 {
    // updates signal if cycle is the correct value
    fn check_cycle(&mut self, cycle: i32, x: i32) {
        let need_to_record = ((cycle - 20) % 40) == 0;
        if need_to_record {
            *self += x * cycle;
        }
    }
}

enum Command {
    NoOp,
    BeginAdd(i32),
    Add(i32),
}

impl FromStr for Command {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Command::NoOp),
            add => {
                let num: i32 = add.split_whitespace().last().unwrap().parse()?;
                Ok(Command::BeginAdd(num))
            }
        }
    }

}


trait ScreenExt {
    fn draw_pixel(&mut self, cycle: i32, x: i32);
}

impl ScreenExt for String {
    fn draw_pixel(&mut self, cycle: i32, x: i32) {
        let crt = (cycle - 1) % 40;
        if crt == x || crt == x + 1 || crt == x - 1 {
            *self += "#";
        } else {
            *self += ".";
        }
        if crt == 39 {
            *self += "\n";
        }
    }
}