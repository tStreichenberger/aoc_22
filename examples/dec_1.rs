use aoc_22::dec_1::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_calories = solution_1()?;

    std::fs::write("IO/dec_1/out_1.txt", max_calories.to_string())?;

    let top_3_calories = solution_2()?;

    std::fs::write("IO/dec_1/out_2.txt", top_3_calories.to_string())?;

    Ok(())
}