/**
 * [39] Combination Sum
 */
pub struct Solution {}


// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        let mut candidates = candidates;
        candidates.sort();

        let mut path = vec![];
        Self::dfs(&candidates, target, &mut path, 0, &mut result);

        result.iter().map(|x| x.clone()).collect()
    }

    fn dfs(candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, sum: i32, result: &mut HashSet<Vec<i32>>) {
        if sum > target {
            return;
        }

        if sum == target {
            let mut p = path.clone();
            p.sort();
            result.insert(p);
            return;
        }

        for &i in candidates {
            if i > target - sum {
                break;
            }

            path.push(i);
            Self::dfs(candidates, target, path, sum + i, result);
            path.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        let mut set = HashSet::new();
        
        set.insert(vec![1,2]);
        set.insert(vec![1,2]);
        
        assert_eq!(set.len(), 1);
    }
}
