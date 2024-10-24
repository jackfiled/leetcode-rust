/**
 * [2846] Minimum Edge Weight Equilibrium Queries in a Tree
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parents = Vec::with_capacity(n);

        for i in 0..n {
            parents.push(i);
        }

        UnionFind { parents }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }

        self.parents[x] = self.find(self.parents[x]);
        self.parents[x]
    }

    fn merge(&mut self, parent: usize, child: usize) {
        self.parents[child] = parent;
    }
}

struct Tree {
    matrix: Vec<HashMap<usize, usize>>,
    // 从节点0到节点i上权值为j边的个数
    count: Vec<Vec<i32>>,
    length: usize,
    // tarjan算法辅助数组
    least_common_ancestor: Vec<usize>,
    // (i, x) 表示边，y 是least_common_ancestor的下标
    query_edges: Vec<Vec<(usize, usize)>>,
    uf: UnionFind,
    visited: Vec<bool>,
}

impl Tree {
    fn new(n: i32, edges: Vec<Vec<i32>>, queries: &Vec<Vec<i32>>) -> Tree {
        let n = n as usize;
        let mut matrix = Vec::with_capacity(n);
        let mut count = Vec::with_capacity(n);

        for _ in 0..=n {
            matrix.push(HashMap::new());
            count.push(vec![0; 27])
        }

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            matrix[x].insert(y, edge[2] as usize);
            matrix[y].insert(x, edge[2] as usize);
        }

        let mut tree = Tree {
            matrix,
            length: n,
            count,
            least_common_ancestor: vec![0; queries.len()],
            query_edges: vec![vec![]; n],
            uf: UnionFind::new(n),
            visited: vec![false; n],
        };

        for (index, query) in queries.iter().enumerate() {
            let x = query[0] as usize;
            let y = query[1] as usize;
            tree.query_edges[x].push((y, index));
            tree.query_edges[y].push((x, index));
        }

        tree
    }

    fn tarjan(&mut self, node: usize, parent: usize) {
        // 顺便构建count数组
        if node != 0 {
            self.count[node] = self.count[parent].clone();
            self.count[node][self.matrix[node][&parent]] += 1;
        }

        let mut next_array = Vec::new();
        for (next, _) in &self.matrix[node] {
            if *next == parent {
                continue;
            }

            next_array.push(*next);
        }

        for next in next_array {
            self.tarjan(next, node);
            self.uf.merge(node, next);
        }

        for (target, index) in &self.query_edges[node] {
            if *target != node && !self.visited[*target] {
                continue;
            }

            self.least_common_ancestor[*index] = self.uf.find(*target);
        }

        self.visited[node] = true;
    }
}

impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut tree = Tree::new(n, edges, &queries);
        tree.tarjan(0, 0);

        let mut result = Vec::new();
        for (index, query) in queries.iter().enumerate() {
            let mut total = 0;
            let mut max_count = 0;
            let (x, y) = (query[0] as usize, query[1] as usize);

            for i in 1..=26 {
                let t = tree.count[x][i] + tree.count[y][i]
                    - 2 * tree.count[tree.least_common_ancestor[index]][i];
                max_count = max_count.max(t);
                total += t;
            }

            result.push(total - max_count);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2846() {
        assert_eq!(
            Solution::min_operations_queries(
                7,
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 1],
                    vec![3, 4, 2],
                    vec![4, 5, 2],
                    vec![5, 6, 2]
                ],
                vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]]
            ),
            vec![0, 0, 1, 3]
        )
    }
}
