use aoc_22::{dec_11::*, AOCResult, io};

fn main() -> AOCResult<()> {
    let input = io::read_in(11)?;

    let ans_1 = solution_1(input.clone())?;
    io::write_out(11, 1, ans_1)?;

    let ans_2 = solution_2(input.clone())?;
    io::write_out(11, 2, ans_2)?;

    Ok(())
}
