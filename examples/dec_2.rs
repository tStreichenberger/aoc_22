use aoc_22::{dec_2::*, AOCResult, io};

fn main() -> AOCResult<()> {
    let input = io::read_in(2)?;

    let total_points = solution_1(input.clone())?;
    io::write_out(2, 1, total_points)?;

    let total_points = solution_2(input.clone())?;
    io::write_out(2, 2, total_points)?;

    Ok(())
}
