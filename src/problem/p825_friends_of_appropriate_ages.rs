/**
 * [825] Friends Of Appropriate Ages
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let n = ages.len();
        let mut ages = ages;
        ages.sort_unstable();

        let mut result = 0;

        for i in (0..n).rev() {
            let lower_bound = ages[i] / 2 + 7;
            if lower_bound >= ages[i] {
                continue;
            }

            let mut j = i + 1;
            while j < n && ages[j] == ages[i] {
                result += 1;
                j += 1;
            }

            match ages.binary_search(&lower_bound) {
                Ok(pos) => {
                    let mut pos = pos;
                    while pos + 1 < n && ages[pos] == ages[pos + 1] {
                        pos += 1;
                    }

                    result += i - pos - 1;
                }
                Err(pos) => {
                    result += i - pos;
                }
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_825() {
        assert_eq!(2, Solution::num_friend_requests(vec![16, 16]));
        assert_eq!(2, Solution::num_friend_requests(vec![16, 17, 18]));
        assert_eq!(
            3,
            Solution::num_friend_requests(vec![20, 30, 100, 110, 120])
        );
    }
}
