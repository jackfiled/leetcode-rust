/**
 * [1870] Minimum Speed to Arrive on Time
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let n = dist.len();

        if hour <= (n - 1) as f64 {
            return -1;
        }

        let max_speed = (*dist.iter().max().unwrap()).max(
            ((dist[n - 1] as f64) / (hour - (n - 1) as f64)).ceil() as i32
        );

        let check = |v: i32| -> bool {
            let mut time = 0;
            for i in 0..n - 1 {
                time += if dist[i] % v == 0 {
                    dist[i] / v
                } else {
                    dist[i] / v + 1
                };
            }

            let time = time as f64 + dist[n - 1] as f64 / v as f64;

            time <= hour
        };

        let (mut left, mut right) = (1, max_speed);

        while left < right {
            let middle = (left + right) / 2;

            if check(middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        right
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1870() {
        assert_eq!(-1, Solution::min_speed_on_time(vec![1, 1], 1.0));
        assert_eq!(10_000_000, Solution::min_speed_on_time(vec![1, 1, 100_000], 2.01));
        assert_eq!(1, Solution::min_speed_on_time(vec![1, 3, 2], 6f64));
        assert_eq!(3, Solution::min_speed_on_time(vec![1, 3, 2], 2.7));
        assert_eq!(-1, Solution::min_speed_on_time(vec![1, 3, 2], 1.9));
    }
}
