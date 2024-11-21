/**
 * [3248] Snake in Matrix
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let (mut x, mut y) = (0, 0);

        commands.iter().for_each(|str| match str.as_str() {
            "UP" => x -= 1,
            "DOWN" => x += 1,
            "LEFT" => y -= 1,
            "RIGHT" => y += 1,
            _ => {}
        });

        x * n + y
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3248() {
        assert_eq!(
            3,
            Solution::final_position_of_snake(2, vec_string!("RIGHT", "DOWN"))
        );
        assert_eq!(
            1,
            Solution::final_position_of_snake(3, vec_string!("DOWN", "RIGHT", "UP"))
        );
    }
}
