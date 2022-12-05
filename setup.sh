echo "Setting Up Environment For December $1th"

example=$"use aoc_22::{dec_$1::*, AOCResult, io};

fn main() -> AOCResult<()> {
    let input = io::read_in(4)?;

    let ans_1 = solution_1(input.clone())?;
    io::write_out($1, 1, ans_1)?;

    let ans_2 = solution_2(input.clone())?;
    io::write_out($1, 2, ans_2)?;

    Ok(())
}"
echo "$example" > examples/dec_$1.rs

mkdir -p IO/dec_$1
touch IO/dec_$1/in.txt


script=$"use crate::AOCResult;

pub fn solution_1(input: String) -> AOCResult<u32> {
    Ok(42)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    Ok(42)
}"

path=src/dec_$1.rs

if ! test -e $path; then
    echo "$script" > src/dec_$1.rs
fi