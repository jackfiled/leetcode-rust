/**
 * [191] Number of 1 Bits
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        static ARRAY: [i32; 31] = [
            1 << 30,
            1 << 29,
            1 << 28,
            1 << 27,
            1 << 26,
            1 << 25,
            1 << 24,
            1 << 23,
            1 << 22,
            1 << 21,
            1 << 20,
            1 << 19,
            1 << 18,
            1 << 17,
            1 << 16,
            1 << 15,
            1 << 14,
            1 << 13,
            1 << 12,
            1 << 11,
            1 << 10,
            1 << 9,
            1 << 8,
            1 << 7,
            1 << 6,
            1 << 5,
            1 << 4,
            1 << 3,
            1 << 2,
            1 << 1,
            1,
        ];

        let mut result = 0;

        for i in 0..31 {
            if n & ARRAY[i] == ARRAY[i] {
                result += 1;
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
    fn test_191() {
        assert_eq!(3, Solution::hamming_weight(11));
        assert_eq!(1, Solution::hamming_weight(128));
    }
}
