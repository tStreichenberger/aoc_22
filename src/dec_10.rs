use crate::AOCResult;

pub fn solution_1(input: String) -> AOCResult<i32> {
    let mut cycle = 1;
    let mut x = 1;
    let mut total_signal = 0;
    for command in input.split("\n") {
        if command != "noop" {
            cycle += 1;
            total_signal.check_cycle(cycle, x);
            let num: i32 = command.split_whitespace().last().unwrap().parse()?;
            x += num;
        }
        cycle += 1;
        total_signal.check_cycle(cycle, x);
    }
    Ok(total_signal)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    Ok(42)
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