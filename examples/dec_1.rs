use aoc_22::dec_1::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("IO/dec_1/in.txt")?;


    let max_calories = solution_1(input.clone())?;
    std::fs::write("IO/dec_1/out_1.txt", max_calories.to_string())?;


    let top_3_calories = solution_2(input)?;
    std::fs::write("IO/dec_1/out_2.txt", top_3_calories.to_string())?;

    Ok(())
}
