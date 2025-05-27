/**
 * [2894] Divisible and Non-divisible Sums Difference
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        (1..=n)
            .filter_map(|x| if x % m != 0 { Some(x) } else { None })
            .sum::<i32>()
            - (1..=n)
                .filter_map(|x| if x % m == 0 { Some(x) } else { None })
                .sum::<i32>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2894() {
        assert_eq!(19, Solution::difference_of_sums(10, 3));
        assert_eq!(15, Solution::difference_of_sums(5, 6));
        assert_eq!(-15, Solution::difference_of_sums(5, 1));
    }
}
