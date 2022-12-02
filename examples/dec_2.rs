use aoc_22::{
    AOCResult,
    dec_2::*,
    IO,
};

fn main() -> AOCResult<()> {
    let input = IO::read_in(2)?;


    let total_points = solution_1(input.clone())?;
    IO::write_out(2, 1, total_points)?;

    let total_points = solution_2(input.clone())?;
    IO::write_out(2, 2, total_points)?;


    Ok(())
}