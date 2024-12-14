/**
 * [3266] Final Array State After K Multiplication Operations II
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Number {
    value: i64,
    pos: usize,
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.value.cmp(&self.value) {
            Ordering::Equal => other.pos.cmp(&self.pos),
            r => r,
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let multiplier = multiplier as i64;

        if multiplier == 1 {
            return nums;
        }

        let n = nums.len();
        let max_number = *nums.iter().max().unwrap() as i64;
        let mut heap = BinaryHeap::new();

        for (i, v) in nums.into_iter().enumerate() {
            heap.push(Number {
                value: v as i64,
                pos: i,
            });
        }

        let mut actual_k = k;
        for _ in 0..k {
            if let Some(head) = heap.peek() {
                if head.value >= max_number {
                    break;
                }
            }

            let head = heap.pop().unwrap();

            heap.push(Number {
                value: head.value * multiplier,
                pos: head.pos,
            });
            actual_k -= 1;
        }

        let mut result = vec![0; n];

        let k = actual_k as usize;
        // 牙医 shake it!
        // 为什么heap.iter()会是以随机的顺序返回！
        for (i, number) in std::iter::from_fn(move || heap.pop()).enumerate() {
            let t = k / n + if i < k % n { 1 } else { 0 };
            result[number.pos] =
                (number.value % MOD * Self::quick_mulitplx(multiplier, t as i64) % MOD) as i32;
        }

        result
    }

    fn quick_mulitplx(mut x: i64, mut y: i64) -> i64 {
        let mut result = 1;

        while y > 0 {
            if y & 1 == 1 {
                result = (result * x) % MOD;
            }

            y = y >> 1;
            x = (x * x) % MOD;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3266() {
        assert_eq!(
            vec![664480092, 763599523, 886046925, 47878852],
            Solution::get_final_state(
                vec![66307295, 441787703, 589039035, 322281864],
                900_900_704,
                641725
            )
        );
        assert_eq!(
            vec![8, 8, 4],
            Solution::get_final_state(vec![4, 2, 4], 3, 2)
        );
        assert_eq!(
            vec![8, 4, 6, 5, 6],
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
        );
        assert_eq!(
            vec![999_999_307, 999_999_993],
            Solution::get_final_state(vec![100_000, 2_000], 2, 1_000_000)
        );
    }
}
