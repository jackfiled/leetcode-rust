/**
 * [2580] Count Ways to Group Overlapping Ranges
 */
pub struct Solution {}

// submission codes start here
struct Range {
    start: i32,
    end: i32,
}

impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut ranges: Vec<Range> = ranges
            .iter()
            .map(|a| {
                return Range {
                    start: a[0],
                    end: a[1],
                };
            })
            .collect();

        ranges.sort_unstable_by_key(|x| x.start);

        let mut merged_ranges = Vec::with_capacity(ranges.len());

        let mut i = 0;
        let mut last: Option<&mut Range> = None;

        while i < ranges.len() {
            if let Some(last_range) = last {
                while i < ranges.len() && last_range.end >= ranges[i].start {
                    last_range.end = last_range.end.max(ranges[i].end);
                    i += 1;
                }

                if i >= ranges.len() {
                    break;
                }
            }

            merged_ranges.push(Range {
                start: ranges[i].start,
                end: ranges[i].end,
            });
            last = merged_ranges.last_mut();
            i += 1;
        }

        let mut result = 1;

        for _ in 0..merged_ranges.len() as i32 {
            result = result * 2 % 1_000_000_007;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2580() {
        assert_eq!(2, Solution::count_ways(vec![vec![6, 10], vec![5, 15]]));
        assert_eq!(
            4,
            Solution::count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]])
        );
    }
}
