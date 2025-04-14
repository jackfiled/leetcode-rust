/**
 * [1534] Count Good Triplets
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;
        let (a, b, c) = (a as u32, b as u32, c as u32);

        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                for k in j + 1..arr.len() {
                    if arr[i].abs_diff(arr[j]) > a {
                        continue;
                    }

                    if arr[j].abs_diff(arr[k]) > b {
                        continue;
                    }

                    if arr[i].abs_diff(arr[k]) > c {
                        continue;
                    }

                    result += 1;
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
    fn test_1534() {
        assert_eq!(
            4,
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
        );
        assert_eq!(
            0,
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1)
        );
    }
}
