/**
 * [2673] Make Costs of Paths Equal in a Binary Tree
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut cost = cost;
        let mut now = (n - 2) as usize;

        loop {
            result += (cost[now] - cost[now + 1]).abs();
            cost[now / 2] += cost[now].max(cost[now + 1]);

            if now <= 2 {
                break;
            }

            now = now - 2;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2673() {
        assert_eq!(Solution::min_increments(7, vec![1,5,2,2,3,3,1]), 6);
    }
}
