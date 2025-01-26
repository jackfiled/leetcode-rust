/**
 * [40] Combination Sum II
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut frequency_map = HashMap::new();

        for c in candidates {
            let entry = frequency_map.entry(c).or_insert(0);
            *entry += 1;
        }

        // (number, count)
        let mut candidates = Vec::from_iter(frequency_map.into_iter());
        candidates.sort();

        let mut path = vec![];
        let mut result = vec![];

        Self::dfs(&candidates, 0, target, &mut path, &mut result);

        result
    }

    fn dfs(
        candidates: &Vec<(i32, i32)>,
        pos: usize,
        rest: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if rest == 0 {
            result.push(path.clone());
            return;
        }

        if pos >= candidates.len() || rest < candidates[pos].0 {
            return;
        }

        // 不选择pos
        Self::dfs(candidates, pos + 1, rest, path, result);

        // 选择pos
        // 能够选择pos最多的次数
        let count = candidates[pos].1.min(rest / candidates[pos].0);
        for i in 1..=count {
            path.push(candidates[pos].0);
            Self::dfs(
                candidates,
                pos + 1,
                rest - candidates[pos].0 * i,
                path,
                result,
            );
        }

        // dfs结束弹出
        for _ in 1..=count {
            path.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn vec_equal_helper(expect: Vec<Vec<i32>>, actual: Vec<Vec<i32>>) {
        let set: HashSet<Vec<i32>> = HashSet::from_iter(expect.into_iter());

        assert_eq!(set.len(), actual.len());

        for array in actual {
            assert!(set.contains(&array));
        }
    }

    #[test]
    fn test_40() {
        vec_equal_helper(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        );

        vec_equal_helper(
            vec![vec![1, 2, 2], vec![5]],
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
        );
    }
}
