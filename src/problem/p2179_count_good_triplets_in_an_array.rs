/**
 * [2179] Count Good Triplets in an Array
 */
pub struct Solution {}

// submission codes start here

struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, mut index: usize, delta: i64) {
        index += 1;

        while index < self.tree.len() {
            self.tree[index] += delta;

            index += Self::low_bit(index)
        }
    }

    fn query(&self, mut index: usize) -> i64 {
        index += 1;
        let mut result = 0;

        while index > 0 {
            result += self.tree[index];
            index -= Self::low_bit(index)
        }

        result
    }

    // 二进制最低位1以及后面的0组成的数
    fn low_bit(i: usize) -> usize {
        i & (!i + 1)
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        // nums2中各个值到下标的映射数组
        let mut pos2 = vec![0; n];
        // index_mapping 是nums1中一个数在nums2中的下标
        // reverse_index_mapping是该下标同nums1中数字下标的对应关系
        let mut reversed_index_mapping = vec![0; n];

        for (i, v) in nums2.iter().enumerate() {
            pos2[*v as usize] = i;
        }

        for i in 0..n {
            // nums1中的一个值在nums2中的下标到该值在nums1中的下标的映射
            reversed_index_mapping[pos2[nums1[i] as usize]] = i;
        }

        let mut tree = FenwickTree::new(n);

        let mut result = 0;

        // 枚举num1s中的一个值在nums2中的下标
        for j in 0..n {
            let pos = reversed_index_mapping[j];
            let left = tree.query(pos);
            tree.update(pos, 1);
            let right = ((n - 1 - pos) - (j - left as usize)) as i64;
            result += left * right;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2179() {
        assert_eq!(
            1,
            Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3])
        );
        assert_eq!(
            4,
            Solution::good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3])
        );
    }
}
