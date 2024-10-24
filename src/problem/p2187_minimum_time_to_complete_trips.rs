/**
 * [2187] Minimum Time to Complete Trips
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let time: Vec<i64> = time.iter().map(|x| *x as i64).collect();
        let total_trips = total_trips as i64;

        let check = |t: i64| -> bool {
            let mut trips = 0;

            for &bus in time.iter() {
                trips += t / bus;
            }

            trips >= total_trips
        };

        let mut left = 0;
        let mut right = time[0] * total_trips;

        while left < right {
            let middle = (right - left) / 2 + left;

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
    fn test_2187() {
        assert_eq!(3, Solution::minimum_time(vec![1, 2, 3], 5));
        assert_eq!(2, Solution::minimum_time(vec![2], 1));
    }
}
