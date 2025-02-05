/**
 * [90] Subsets II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(2usize.pow(nums.len() as u32));
        let mut path = Vec::with_capacity(nums.len());

        nums.sort_unstable();
        Self::search(&nums, &mut path, 0, false, &mut result);

        result
    }

    pub fn search(
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        pos: usize,
        choose: bool,
        result: &mut Vec<Vec<i32>>,
    ) {
        if pos == nums.len() {
            result.push(path.clone());
            return;
        }

        // 不选择pos
        Self::search(nums, path, pos + 1, false, result);

        // 选择pos
        // 如果nums[pos - 1] == nums[pos - 1] 且没有选择前一个数
        // 说明选择重复了
        if !choose && pos > 0 && nums[pos - 1] == nums[pos] {
            return;
        } else {
            path.push(nums[pos]);
            Self::search(nums, path, pos + 1, true, result);
            path.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        assert_eq!(
            vec![
                vec![],
                vec![2],
                vec![2, 2],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
            ],
            Solution::subsets_with_dup(vec![1, 2, 2])
        );
        assert_eq!(vec![vec![], vec![0]], Solution::subsets_with_dup(vec![0]));
    }
}
