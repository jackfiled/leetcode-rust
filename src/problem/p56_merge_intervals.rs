/**
 * [56] Merge Intervals
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<(i32, i32)> = intervals.iter().map(|p| (p[0], p[1])).collect();

        intervals.sort_unstable();

        let mut result: Vec<(i32, i32)> = vec![intervals[0]];
        let mut last = 0;

        for i in 1..intervals.len() {
            let (begin, end) = intervals[i];
            let (last_begin, last_end) = result[last];

            if begin <= last_end {
                result[last].0 = result[last].0.min(begin);
                result[last].1 = result[last].1.max(end);
            } else {
                result.push(intervals[i]);
                last += 1;
            }
        }

        result.iter().map(|p| vec![p.0, p.1]).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56() {
        assert_eq!(
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        )
    }
}
