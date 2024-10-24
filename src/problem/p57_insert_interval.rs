/**
 * [57] Insert Interval
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut left, mut right) = (new_interval[0], new_interval[1]);

        let mut result = vec![];
        let mut placed = false;

        for interval in intervals {
            if interval[1] < left {
                result.push(interval);
            } else if interval[0] > right {
                if !placed {
                    result.push(vec![left, right]);
                    placed = true;
                }

                result.push(interval);
            } else {
                left = left.min(interval[0]);
                right = right.max(interval[1]);
            }
        }

        if !placed {
            result.push(vec![left, right]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_57() {}
}
