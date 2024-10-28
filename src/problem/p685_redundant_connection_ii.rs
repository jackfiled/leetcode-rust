/**
 * [685] Redundant Connection II
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
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut set = UnionFindSet::new(n + 1);
        let mut parents: Vec<usize> = (0..=n).collect();

        let (mut conflict, mut cycle) = (usize::MAX, usize::MAX);

        for (i, edge) in edges.iter().enumerate() {
            let (first, second) = (edge[0] as usize, edge[1] as usize);

            if parents[second] != second {
                // There are the parents for second
                conflict = i;
            } else {
                parents[second] = first;

                if set.find(first) == set.find(second) {
                    // There are a cycle between first and second.
                    cycle = i;
                } else {
                    set.merge(first, second);
                }
            }
        }

        if conflict == usize::MAX {
            edges[cycle].clone()
        } else {
            let result = edges[conflict].clone();

            if cycle == usize::MAX {
                result
            } else {
                vec![parents[result[1] as usize] as i32, result[1]]
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_685() {
        assert_eq!(
            vec![2, 3],
            Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
        );
        assert_eq!(
            vec![4, 1],
            Solution::find_redundant_directed_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 5]
            ])
        );
    }
}
