use std::ffi::FromVecWithNulError;

/**
 * [2209] Minimum White Tiles After Covering With Carpets
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let carpets_num = num_carpets as usize;
        let carpet_length = carpet_len as usize;
        let floor: Vec<bool> = floor
            .chars()
            .into_iter()
            .map(|x| if x == '0' { true } else { false })
            .collect();

        // dp[i][j] 表示
        // 当使用j块地毯时
        // [i..] 还有多少白色的地砖没有被覆盖
        let mut dp = vec![vec![0; carpets_num + 1]; floor.len()];

        // 初始化dp[i][0]
        let mut white_count = 0;
        for i in (0..floor.len()).rev() {
            if !floor[i] {
                white_count += 1;
            }
            dp[i][0] = white_count;
        }

        for i in (0..floor.len() - 1).rev() {
            for j in 1..=carpets_num {
                // 在floor[i]处不放置地毯
                let a = dp[i + 1][j] + if !floor[i] { 1 } else { 0 };
                // 在floor[i]处放置地毯
                // 如果放置的长度超过地板的总长
                // 则剩余的白色地砖数量必然为0
                let b = if i + carpet_length >= floor.len() {
                    0
                } else {
                    dp[i + carpet_length][j - 1]
                };
                dp[i][j] = a.min(b);
            }
        }

        dp[0][carpets_num]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2209() {
        assert_eq!(0, Solution::minimum_white_tiles("11111".to_owned(), 2, 3));
        assert_eq!(
            2,
            Solution::minimum_white_tiles("10110101".to_owned(), 2, 2)
        );
    }
}
