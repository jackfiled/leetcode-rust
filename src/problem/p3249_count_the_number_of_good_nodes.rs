/**
 * [3249] Count the Number of Good Nodes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut matrix = vec![vec![]; n];

        edges
            .into_iter()
            .map(|edge| (edge[0] as usize, edge[1] as usize))
            .for_each(|(x, y)| {
                matrix[x].push(y);
                matrix[y].push(x);
            });

        let mut result = 0;
        Self::dfs(&matrix, 0, usize::MAX, &mut result);
        result
    }

    fn dfs(tree: &Vec<Vec<usize>>, node: usize, parent: usize, result: &mut i32) -> i32 {
        if tree[node].len() == 0 {
            *result += 1;
            return 1;
        }

        let children: Vec<i32> = tree[node]
            .iter()
            .filter_map(|&child| {
                if child == parent {
                    None
                } else {
                    Some(Self::dfs(tree, child, node, result))
                }
            })
            .collect();

        if children.iter().all(|x| *x == children[0]) {
            *result += 1;
        }

        children.into_iter().sum::<i32>() + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3249() {
        assert_eq!(
            7,
            Solution::count_good_nodes(vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6]
            ])
        );

        assert_eq!(
            6,
            Solution::count_good_nodes(vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![0, 5],
                vec![1, 6],
                vec![2, 7],
                vec![3, 8]
            ])
        );

        assert_eq!(
            6,
            Solution::count_good_nodes(vec![
                vec![6, 0],
                vec![1, 0],
                vec![5, 1],
                vec![2, 5],
                vec![3, 1],
                vec![4, 3]
            ])
        );
    }
}
