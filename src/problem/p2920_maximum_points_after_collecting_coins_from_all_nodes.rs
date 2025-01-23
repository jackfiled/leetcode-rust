/**
 * [2920] Maximum Points After Collecting Coins From All Nodes
 */
pub struct Solution {}

// submission codes start here
use std::cell::RefCell;

struct Search {
    memory: RefCell<Vec<Vec<i32>>>,
    children: Vec<Vec<usize>>,
    coins: Vec<i32>,
}

impl Search {
    fn new(children: Vec<Vec<usize>>, n: usize, coins: Vec<i32>) -> Self {
        Self {
            children,
            // 14是树可能的最大高度
            memory: RefCell::new(vec![vec![-1; 14]; n]),
            coins,
        }
    }

    fn search(&self, node: usize, parent: usize, f: usize, k: i32) -> i32 {
        if self.memory.borrow()[node][f] >= 0 {
            return self.memory.borrow()[node][f];
        }

        let mut result0 = (self.coins[node] >> f) - k;
        let mut result1 = self.coins[node] >> (f + 1);

        for &child in self.children[node].iter() {
            if child == parent {
                continue;
            }

            result0 += self.search(child, node, f, k);
            if f + 1 < 14 {
                result1 += self.search(child, node, f + 1, k);
            }
        }

        self.memory.borrow_mut()[node][f] = result0.max(result1);
        self.memory.borrow()[node][f]
    }
}

impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let mut children = vec![vec![]; n];

        for edge in edges {
            let (x, y) = (edge[0] as usize, edge[1] as usize);

            children[x].push(y);
            children[y].push(x);
        }

        let search = Search::new(children, n, coins);

        search.search(0, usize::MAX, 0, k)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2920() {
        assert_eq!(
            11,
            Solution::maximum_points(
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                vec![10, 10, 3, 3],
                5
            )
        );
    }
}
