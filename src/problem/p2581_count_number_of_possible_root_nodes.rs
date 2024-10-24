use std::usize;

/**
 * [2581] Count Number of Possible Root Nodes
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut graph = vec![vec![]; edges.len() + 1];

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            graph[x].push(y);
            graph[y].push(x);
        }

        let mut guess_set = HashSet::with_capacity(guesses.len());
        for guess in guesses {
            guess_set.insert(Solution::hash(guess[0] as usize, guess[1] as usize));
        }

        let mut count = 0;
        Solution::dfs(&graph, &guess_set, 0, usize::MAX, &mut count);
        dbg!(count);

        let mut result = 0;
        Solution::tree_dp(&graph, &guess_set, &k, 0, usize::MAX, count, &mut result);

        result
    }

    fn hash(x: usize, y: usize) -> i64 {
        (x as i64) * 1000000 + (y as i64)
    }

    fn dfs(
        graph: &Vec<Vec<usize>>,
        guess_set: &HashSet<i64>,
        now: usize,
        pre: usize,
        count: &mut i32,
    ) {
        for next in &graph[now] {
            let next = *next;
            if next == pre {
                continue;
            }

            if guess_set.contains(&Solution::hash(now, next)) {
                *count += 1;
            }

            Solution::dfs(graph, guess_set, next, now, count);
        }
    }

    fn tree_dp(
        graph: &Vec<Vec<usize>>,
        guess_set: &HashSet<i64>,
        k: &i32,
        now: usize,
        pre: usize,
        count: i32,
        result: &mut i32,
    ) {
        if count >= *k {
            *result += 1;
        }

        for next in &graph[now] {
            let next = *next;
            if next == pre {
                continue;
            }

            let mut count = count;
            if guess_set.contains(&Solution::hash(now, next)) {
                count -= 1;
            }

            if guess_set.contains(&Solution::hash(next, now)) {
                count += 1;
            }

            Solution::tree_dp(graph, guess_set, k, next, now, count, result);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2581() {
        assert_eq!(
            Solution::root_count(
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]],
                vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]],
                3
            ),
            3
        );

        assert_eq!(
            Solution::root_count(
                vec![
                    vec![0, 1],
                    vec![2, 0],
                    vec![0, 3],
                    vec![4, 2],
                    vec![3, 5],
                    vec![6, 0],
                    vec![1, 7],
                    vec![2, 8],
                    vec![2, 9],
                    vec![4, 10],
                    vec![9, 11],
                    vec![3, 12],
                    vec![13, 8],
                    vec![14, 9],
                    vec![15, 9],
                    vec![10, 16]
                ],
                vec![vec![8, 2], vec![12, 3], vec![0, 1], vec![16, 10]],
                2
            ),
            4
        );
    }
}
