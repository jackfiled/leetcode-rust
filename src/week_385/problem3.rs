pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        let direction = vec![
            (0, 1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        let mut result = HashMap::new();

        let n = mat.len() as i32;
        let m = mat[0].len() as i32;

        for i in 0..n {
            for j in 0..m {
                for d in &direction {
                    let mut num = 0;
                    let mut step = 0;

                    loop {
                        let x = (i + d.0 * step);
                        let y = (j + d.1 * step);
                        step += 1;

                        if x < 0 || x >= n || y < 0 || y >= m {
                            break;
                        }

                        num = num * 10 + mat[x as usize][y as usize];

                        if num > 10 {
                            if let Some(entry) = result.get_mut(&num) {
                                *entry += 1;
                            } else if Solution::is_prime(num) {
                                result.insert(num, 1);
                            }
                        }
                    }
                }
            }
        }

        let (mut best, mut num) = (-1, 0);

        for (key, value) in result {
            if num < value || (num == value && key > best) {
                best = key;
                num = value;
            }
        }

        best
    }

    fn is_prime(num: i32) -> bool {
        let mut i = 2;
        while i * i <= num {
            if num % i == 0 {
                return false;
            }
            i += 1;
        }

        true
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_385_3() {
        assert_eq!(
            19,
            Solution::most_frequent_prime(vec![vec![1, 1], vec![9, 9], vec![1, 1]])
        );
        assert_eq!(
            29,
            Solution::most_frequent_prime(vec![vec![2], vec![9], vec![6]])
        );
    }
}
