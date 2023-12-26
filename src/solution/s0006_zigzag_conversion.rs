/**
 * [6] Zigzag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 * 
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 * 
 * string convert(string s, int numRows);
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "A", numRows = 1
 * Output: "A"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/zigzag-conversion/
// discuss: https://leetcode.cn/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let length = s.len() as i32;
        if num_rows < 2 {
            return s;
        }

        let mut result = String::new();
        let s: Vec<char> = s.chars().collect();

        for row in 0..num_rows {
            for begin in (0..length).step_by((2 * num_rows - 2) as usize) {
                // 竖线上的元素
                let i = begin + row;

                if i >= length {
                    break;
                }
                result.push(s[i as usize]);

                // 斜线上的元素
                let j = i + 2 * (num_rows - row - 1);
                if j >= length {
                    break;
                } else if j == i + 2 * (num_rows - 1) || i == j {
                    continue;
                }
                result.push(s[j as usize]);
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
    fn test_6() {
        assert_eq!(String::from("PAHNAPLSIIGYIR"), Solution::convert(
            String::from("PAYPALISHIRING"), 3
        ));
    }
}
