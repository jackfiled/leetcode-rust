/**
 * [47] Permutations II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut result = vec![];
        let mut visited = vec![false; nums.len()];
        let mut path = Vec::with_capacity(nums.len());

        Self::search(&nums, 0, &mut path, &mut visited, &mut result);

        result
    }

    fn search(
        nums: &Vec<i32>,
        pos: usize,
        path: &mut Vec<i32>,
        visited: &mut Vec<bool>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if pos == nums.len() {
            result.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            if visited[i] || (i > 0 && nums[i - 1] == nums[i] && !visited[i - 1]) {
                continue;
            }

            visited[i] = true;

            path.push(nums[i]);
            Self::search(nums, pos + 1, path, visited, result);
            path.pop();

            visited[i] = false;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
            Solution::permute_unique(vec![1, 1, 2])
        );
    }
}
