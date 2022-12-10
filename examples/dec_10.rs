use aoc_22::{dec_10::*, AOCResult, io};

fn main() -> AOCResult<()> {
    let input = io::read_in(10)?;

    let ans_1 = solution_1(input.clone())?;
    io::write_out(10, 1, ans_1)?;

    let ans_2 = solution_2(input.clone())?;
    io::write_out(10, 2, ans_2)?;

    Ok(())
}
