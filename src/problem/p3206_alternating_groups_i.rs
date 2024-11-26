/**
 * [3206] Alternating Groups I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let n = colors.len();

        let mut result = 0;

        for i in n..2 * n {
            let left = (i - 1) % n;
            let middle = i % n;
            let right = (i + 1) % n;

            if colors[left] != colors[middle] && colors[right] != colors[middle] {
                result += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3206() {
        assert_eq!(0, Solution::number_of_alternating_groups(vec![0, 0, 0]));
        assert_eq!(
            3,
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1])
        );
    }
}
