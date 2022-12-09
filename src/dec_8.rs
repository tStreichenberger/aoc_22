use crate::AOCResult;
use std::collections::HashSet;

pub fn solution_1(input: String) -> AOCResult<usize> {
    let forest = parse_forest(input);
    let mut hidden_from_left = HashSet::new();
    let mut hidden_from_right = HashSet::new();

    let num_rows = forest.len();
    let num_cols = forest[0].len();

    // go down every row
    for row_i in 0..num_rows {
        let mut largest = forest[row_i][0];
        // if row_i == 1 {println!("{largest}")}
        // go to the right on each row
        for col_i in 1..num_cols {
            let tree = forest[row_i][col_i];
            if tree > largest {
                largest = tree;
            } else {
                hidden_from_left.insert((row_i,col_i));
            }
        }
        let mut largest = forest[row_i][num_cols - 1];
        // go to the left on each row
        for col_i in (0..num_cols-1).rev() {
            let tree = forest[row_i][col_i];
            if tree > largest {
                largest = tree;
            } else {
                hidden_from_right.insert((row_i,col_i));
            }
        }
    }

    let mut hidden_from_top = HashSet::new();
    let mut hidden_from_bottom = HashSet::new();
    // iterate over each column
    for col_i in 0..num_cols {
        let mut largest = forest[0][col_i];
        if col_i == 4 {println!("{largest}")}
        // iterate down each column
        for row_i in 1..num_rows {
            let tree = forest[row_i][col_i];
            if tree > largest {
                largest = tree;
            } else {
                hidden_from_top.insert((row_i,col_i));
            }
        }
        let mut largest = forest[num_rows - 1][col_i];
        // iterator up each column
        for row_i in (0..num_rows-1).rev() {
            let tree = forest[row_i][col_i];
            if tree > largest {
                largest = tree;
            } else {
                hidden_from_bottom.insert((row_i,col_i));
            }
        }
    }
    let visible = (1,2);
    println!("Hidden From Top?: {}", hidden_from_top.contains(&visible));

    // find trees that are hidden in all directions
    let hidden_vert: HashSet<&(usize,usize)> = hidden_from_top.intersection(&hidden_from_bottom).collect();
    let hidden_horiz: HashSet<&(usize,usize)> = hidden_from_left.intersection(&hidden_from_right).collect();

    let all_hidden = hidden_vert.intersection(&hidden_horiz);
    // println!("{:?}", all_hidden);
    let border = 5;
    for coord in all_hidden.clone() {
        if coord.0 < border && coord.1 < border {println!("{coord:?}")}
    }
    let total_hidden = all_hidden.count();
    Ok(total_hidden)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    Ok(42)
}



fn parse_forest(input: String) -> Vec<Vec<u32>> {
    let mut forest = Vec::new();
    for line in input.split("\n") {
        let mut row = Vec::new();
        for char in line.chars() {
            let tree = char.to_digit(10).unwrap();
            row.push(tree);
        }
        forest.push(row);
    }
    forest
}

#[test]
fn test() {
    for i in (0..10-1).rev() {
        println!("{i}");
    }
}