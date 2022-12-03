use aoc_22::dec_1::*;
use aoc_22::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::read_in(1)?;

    let max_calories = solution_1(input.clone())?;
    io::write_out(1, 1, max_calories)?;

    let top_3_calories = solution_2(input)?;
    io::write_out(1, 2, top_3_calories)?;

    Ok(())
}
