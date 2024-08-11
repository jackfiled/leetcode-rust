/**
 * [97] Interleaving String
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1, s2, s3): (Vec<char>, Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect(), s3.chars().collect());
        let (n, m, t) = (s1.len(), s2.len(), s3.len());
        
        if n + m != t {
            return false;
        }
        
        let mut dp = vec![false; m + 1];
        dp[0] = true;
        
        for i in 0..=n {
            for j in 0..=m {
                let p = i + j - 1;
                
                if i != 0 {
                    dp[j] &= s1[i - 1] == s3[p];
                }
                
                if j != 0 {
                    dp[j] |= dp[j - 1] && s2[j - 1] == s3[p];
                }
            }
        }
        
        dp[m]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_97() {
    }
}
