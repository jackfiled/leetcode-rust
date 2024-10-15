/**
 * [3200] Maximum Height of a Triangle
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let mut result = 0;
        let mut row = 1;
        let mut switch = true;
        let (mut first_count, mut second_count) = (0, 0);

        loop {
            if switch {
                first_count += row;
            } else {
                second_count += row;
            }

            if first_count > red || second_count > blue {
                if second_count > red || first_count > blue {
                    break;
                }
            }

            switch = !switch;
            row += 1;
            result += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3200() {
        assert_eq!(3, Solution::max_height_of_triangle(2, 4));
        assert_eq!(2, Solution::max_height_of_triangle(2, 1));
        assert_eq!(1, Solution::max_height_of_triangle(1, 1));
        assert_eq!(2, Solution::max_height_of_triangle(10, 1));
        assert_eq!(5, Solution::max_height_of_triangle(10, 10));
    }
}
