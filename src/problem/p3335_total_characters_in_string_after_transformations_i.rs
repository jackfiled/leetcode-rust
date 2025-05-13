/**
 * [3335] Total Characters in String After Transformations I
 */
pub struct Solution {}

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut map = vec![0; 26];

        for c in s.bytes() {
            map[(c - b'a') as usize] += 1;
        }

        let mut result = s.bytes().len() as i32;

        for _ in 0..t {
            let z_count = map[25];

            for c in (0..25).rev() {
                if map[c] != 0 {
                    map[c + 1] = (map[c + 1] + map[c]) % MOD;
                    map[c] = 0;
                }
            }

            if z_count != 0 {
                map[0] = (map[0] + z_count) % MOD;
                map[1] = (map[1] + z_count) % MOD;
                result = (result + z_count) % MOD;

                map[25] = (map[25] + MOD - z_count) % MOD;
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
    fn test_3335() {
        assert_eq!(
            7,
            Solution::length_after_transformations("abcyy".to_string(), 2)
        );
        assert_eq!(
            5,
            Solution::length_after_transformations("azbk".to_string(), 1)
        );
        assert_eq!(
            79033769,
            Solution::length_after_transformations("jqktcurgdvlibczdsvnsg".to_string(), 7517)
        );
    }
}
