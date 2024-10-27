/**
 * [684] Redundant Connection
 */
pub struct Solution {}

// submission codes start here

struct UnionFindSet {
    parents: Vec<usize>,
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..=n).collect(),
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.parents[n] == n {
            n
        } else {
            self.parents[n] = self.find(self.parents[n]);

            self.parents[n]
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));

        self.parents[a] = b;
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut set = UnionFindSet::new(n + 1);

        let mut result = None;
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            if set.find(a) == set.find(b) {
                result = Some(vec![a as i32, b as i32]);
            }

            set.merge(a, b);
        }

        result.unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_684() {
        assert_eq!(
            vec![2, 3],
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
        );
        assert_eq!(
            vec![1, 4],
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ])
        );
    }
}
