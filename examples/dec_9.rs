use aoc_22::{dec_9::*, AOCResult, io};

fn main() -> AOCResult<()> {
    let input = io::read_in(9)?;

    let ans_1 = solution_1(input.clone())?;
    io::write_out(9, 1, ans_1)?;

    let ans_2 = solution_2(input.clone())?;
    io::write_out(9, 2, ans_2)?;

    Ok(())
}
