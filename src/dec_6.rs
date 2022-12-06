use crate::AOCResult;
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn solution_1(input: String) -> AOCResult<u32> {
    let mut chars = input.chars();
    let mut previous_four = VecDeque::new();
    let mut i = 4;
    for _ in 0..4 {previous_four.push_back(chars.next().unwrap());} // we start by adding the first four chars
    if check_unique(&previous_four) {return Ok(i);}
    i += 1; // we start on index 5
    for char in chars {
        previous_four.pop_front();
        previous_four.push_back(char);
        if check_unique(&previous_four) {return Ok(i);}
        i+=1;
    }
    Ok(42)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    let mut chars = input.chars();
    let mut previous_fourteen = VecDeque::new();
    let mut i = 14;
    for _ in 0..14 {previous_fourteen.push_back(chars.next().unwrap());} // we start by adding the first fourteen chars
    if check_unique(&previous_fourteen) {return Ok(i);}
    i += 1; // we start on index 15
    for char in chars {
        previous_fourteen.pop_front();
        previous_fourteen.push_back(char);
        if check_unique(&previous_fourteen) {return Ok(i);}
        i+=1;
    }
    Ok(42)
}




fn check_unique(deq: &VecDeque<char>) -> bool {
    let mut all_unique = true;
    let mut set = HashSet::new();
    for char in deq.iter() {
        all_unique = set.insert(char);
        if !all_unique {return all_unique;}
    }
    return all_unique;
}