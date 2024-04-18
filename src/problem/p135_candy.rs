/**
 * [135] Candy
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut children = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                children[i] = children[i].max(children[i - 1] + 1);
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                children[i] = children[i].max(children[i + 1] + 1);
            }
        }

        children.iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_135() {
        assert_eq!(5, Solution::candy(vec![1, 0, 2]));
        assert_eq!(4, Solution::candy(vec![1, 2, 2]));
    }
}
