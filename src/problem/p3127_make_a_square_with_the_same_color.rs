/**
 * [3127] Make a Square with the Same Color
 */
pub struct Solution {}


// submission codes start here

const DELTA: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0.. 2 {
                let mut white: i32 = 0;
                let mut black: i32 = 0;
                
                for (x, y) in DELTA {
                    match grid[i + x][j + y] {
                        'B' => black += 1,
                        'W' => white += 1,
                        _ => {}
                    }
                }
                
                if white.abs_diff(black) == 2 || white.abs_diff(black) == 4 {
                    return true;
                }
            }
        }
        
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3127() {
    }
}
