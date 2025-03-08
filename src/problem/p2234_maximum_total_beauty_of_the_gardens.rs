/**
 * [2234] Maximum Total Beauty of the Gardens
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_beauty(
        mut flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        let n = flowers.len();
        let target = target as i64;
        let full = full as i64;
        let partial = partial as i64;

        let mut flowers: Vec<i64> = flowers
            .into_iter()
            .map(|x| (x as i64).min(target))
            .collect();

        flowers.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = 0;
        let mut sum: i64 = flowers.iter().map(|x| *x).sum();
        if target * n as i64 - sum <= new_flowers {
            result += full * n as i64;
        }

        let mut prefix = 0;
        let mut ptr = 0;

        for i in 0..n {
            if i != 0 {
                prefix += flowers[i - 1];
            }

            if flowers[i] == target {
                continue;
            }

            let mut rest = new_flowers - (target * i as i64 - prefix);
            if rest < 0 {
                break;
            }

            while !(ptr >= i && flowers[ptr] * (n - ptr) as i64 - sum <= rest) {
                sum -= flowers[ptr];
                ptr += 1;
            }

            rest -= flowers[ptr] * (n - ptr) as i64 - sum;
            result = result.max(
                full * i as i64
                    + partial * (flowers[ptr] + rest / (n - ptr) as i64).min(target - 1),
            );
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2234() {
        assert_eq!(14, Solution::maximum_beauty(vec![1, 3, 1, 1], 7, 6, 12, 1));
        assert_eq!(30, Solution::maximum_beauty(vec![2, 4, 5, 3], 10, 5, 2, 6));
    }
}
