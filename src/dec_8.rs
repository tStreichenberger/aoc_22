use crate::AOCResult;

pub fn solution_1(input: String) -> AOCResult<usize> {
    let forest = parse_forest(input);
    let mut total_hidden = 0;
    for row in 0..forest.len() {
        for col in 0..forest[0].len() {
            if forest.tree_visible(row, col) {total_hidden += 1}
        }
    }
    Ok(total_hidden)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    let forest = parse_forest(input);
    let mut best = 0;
    for row in 0..forest.len() {
        for col in 0..forest[0].len() {
            let score = forest.tree_score(row, col);
            best = std::cmp::max(score,best);
        }
    }
    Ok(best)
}



fn parse_forest(input: String) -> Forest {
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

type Forest = Vec<Vec<u32>>;

trait ForestExt {
    fn tree_visible(&self, row: usize, col: usize) -> bool;
    fn tree_score(&self, row: usize, col: usize) -> u32;
}

impl ForestExt for Forest {
    fn tree_visible(&self, row: usize, col: usize) -> bool {
        let tree = self[row][col];
        let mut hiddenUp = false;
        if row > 0 {
            let mut up = row - 1;
            while up >= 0 {
                if self[up][col] >= tree {
                    hiddenUp = true;
                    break;
                }
                if up == 0 {break};
                up -= 1;
            }
        }
        let mut down = row + 1;
        let mut hiddenDown = false;
        while down < self.len() {
            if self[down][col] >= tree {
                hiddenDown = true;
                break;
            }
            down += 1;
        }

        let mut hiddenLeft = false;
        if col > 0 {
            let mut left = col - 1;
            while left >= 0 {
                if self[row][left] >= tree {
                    hiddenLeft = true;
                    break;
                }
                if left == 0 {break};
                left -= 1;
            }
        }
        let mut right = col + 1;
        let mut hiddenRight = false;
        while right < self[0].len() {
            if self[row][right] >= tree {
                hiddenRight = true;
                break;
            }
        right += 1;
        }
        ! (hiddenRight && hiddenLeft && hiddenDown && hiddenUp)
    }



    fn tree_score(&self, row: usize, col: usize) -> u32 {
        let tree = self[row][col];
        let mut score = (0,0,0,0);
        if row > 0 {
            let mut up = row - 1;
            while up >= 0 {
                score.0 += 1;
                if self[up][col] >= tree {
                    break;
                }
                if up == 0 {break};
                up -= 1;
            }
        }
        let mut down = row + 1;
        while down < self.len() {
            score.1 += 1;
            if self[down][col] >= tree {
                break;
            }
            down += 1;
        }

        if col > 0 {
            let mut left = col - 1;
            while left >= 0 {
                score.2 +=1;
                if self[row][left] >= tree {
                    break;
                }
                if left == 0 {break};
                left -= 1;
            }
        }
        let mut right = col + 1;
        while right < self[0].len() {
            score.3 +=1;
            if self[row][right] >= tree {
                break;
            }
        right += 1;
        }
        score.0 * score.1 * score.2 * score.3
    }
}