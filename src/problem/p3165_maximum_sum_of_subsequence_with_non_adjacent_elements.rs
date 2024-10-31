/**
 * [3165] Maximum Sum of Subsequence With Non-adjacent Elements
 */
pub struct Solution {}

// submission codes start here

#[derive(Clone)]
struct SegementNode {
    pub v00: i64,
    pub v01: i64,
    pub v10: i64,
    pub v11: i64,
}

impl SegementNode {
    fn new() -> Self {
        Self {
            v00: 0,
            v01: 0,
            v10: 0,
            v11: 0,
        }
    }

    fn set(&mut self, v: i64) {
        self.v00 = 0;
        self.v01 = 0;
        self.v10 = 0;
        self.v11 = v.max(0);
    }
}

struct SegementTree {
    n: usize,
    tree: Vec<SegementNode>,
}

impl SegementTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree: (0..4 * n + 1).map(|_| SegementNode::new()).collect(),
        }
    }

    fn push_up(&mut self, pos: usize) {
        let (left, right) = (self.tree[pos * 2].clone(), self.tree[pos * 2 + 1].clone());

        self.tree[pos].v00 = (left.v00 + right.v10).max(left.v01 + right.v00);
        self.tree[pos].v01 = (left.v00 + right.v11).max(left.v01 + right.v01);
        self.tree[pos].v10 = (left.v10 + right.v10).max(left.v11 + right.v00);
        self.tree[pos].v11 = (left.v10 + right.v11).max(left.v11 + right.v01);
    }

    fn init_recurisely(&mut self, nums: &Vec<i32>, pos: usize, left: usize, right: usize) {
        if left == right {
            self.tree[pos].set(nums[left - 1] as i64);
            return;
        }

        let middle = (left + right) / 2;
        self.init_recurisely(nums, pos * 2, left, middle);
        self.init_recurisely(nums, pos * 2 + 1, middle + 1, right);
        self.push_up(pos);
    }

    fn init(&mut self, nums: Vec<i32>) {
        self.init_recurisely(&nums, 1, 1, self.n);
    }

    fn update_recurisely(&mut self, x: usize, l: usize, r: usize, pos: usize, v: i64) {
        if l > pos || r < pos {
            return;
        }

        if l == r {
            self.tree[x].set(v);
            return;
        }

        let middle = (l + r) / 2;
        self.update_recurisely(x * 2, l, middle, pos, v);
        self.update_recurisely(x * 2 + 1, middle + 1, r, pos, v);
        self.push_up(x);
    }

    fn update(&mut self, query: Vec<i32>) {
        self.update_recurisely(1, 1, self.n, query[0] as usize + 1, query[1] as i64);
    }
}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut tree = SegementTree::new(n);
        tree.init(nums);

        let mut result = 0i64;

        for query in queries {
            tree.update(query);
            result = (result + tree.tree[1].v11) % MOD;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3165() {
        assert_eq!(
            21,
            Solution::maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]])
        );
        assert_eq!(
            0,
            Solution::maximum_sum_subsequence(vec![0, -1], vec![vec![0, -5]])
        );
    }
}
