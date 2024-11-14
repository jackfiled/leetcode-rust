/**
 * [3261] Count Substrings That Satisfy K-Constraint II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let s: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let n = s.len();

        let mut window = (0, 0);
        let mut right_array = vec![n; n];
        let mut prefix = vec![0; n + 1];

        let mut left = 0;
        for right in 0..n {
            if s[right] == 0 {
                window.0 += 1;
            } else {
                window.1 += 1;
            }

            while window.0 > k && window.1 > k {
                if s[left] == 0 {
                    window.0 -= 1;
                } else {
                    window.1 -= 1;
                }
                right_array[left] = right;
                left += 1;
            }

            prefix[right + 1] = prefix[right] + (right - left + 1) as i64;
        }

        queries
            .into_iter()
            .map(|query| {
                let (l, r) = (query[0] as usize, query[1] as usize);
                let min_r = right_array[l].min(r + 1) as i64;
                let l = l as i64;

                (min_r - l + 1) * (min_r - l) / 2 + prefix[r + 1] - prefix[min_r as usize]
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3261() {
        assert_eq!(
            vec![26],
            Solution::count_k_constraint_substrings("0001111".to_owned(), 2, vec![vec![0, 6]])
        );
        assert_eq!(
            vec![15, 9, 3],
            Solution::count_k_constraint_substrings(
                "010101".to_owned(),
                1,
                vec![vec![0, 5], vec![1, 4], vec![2, 3]]
            )
        );
        assert_eq!(
            vec![1, 3, 1],
            Solution::count_k_constraint_substrings(
                "00".to_owned(),
                1,
                vec![vec![0, 0], vec![0, 1], vec![1, 1]]
            )
        );
    }
}
