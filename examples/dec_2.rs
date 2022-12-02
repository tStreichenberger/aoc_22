use aoc_22::{
    AOCResult,
    dec_2::*
};

fn main() -> AOCResult<()> {
    let input = std::fs::read_to_string("IO/dec_2/in.txt")?;


    let total_points = solution_1(input.clone())?;
    std::fs::write("IO/dec_2/out_1.txt", total_points.to_string())?;

    let total_points = solution_2(input.clone())?;
    std::fs::write("IO/dec_2/out_2.txt", total_points.to_string())?;


    Ok(())
}