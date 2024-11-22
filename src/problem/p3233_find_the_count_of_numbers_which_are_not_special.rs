/**
 * [3233] Find the Count of Numbers Which Are Not Special
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let n = (r as f64).sqrt() as usize;
        let mut v = vec![true; n + 1];
        let mut result = r - l + 1;

        let (l, r) = (l as usize, r as usize);
        for i in 2..=n {
            if v[i] {
                if i.pow(2) >= l && i.pow(2) <= r {
                    result -= 1;
                }

                for j in ((i * 2)..=n).step_by(i) {
                    v[j] = false;
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3233() {
        assert_eq!(19, Solution::non_special_count(5, 25));
        assert_eq!(3, Solution::non_special_count(5, 7));
        assert_eq!(11, Solution::non_special_count(4, 16));
    }
}
