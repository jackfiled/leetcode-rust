/**
 * [228] Summary Ranges
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = vec![];

        if nums.len() == 0 {
            return result;
        }

        let mut begin = nums[0];
        let mut end = nums[0];

        for i in 1..nums.len() {
            if nums[i] == end + 1 {
                end += 1;
            } else {
                result.push(if begin == end {
                    format!("{}", begin)
                } else {
                    format!("{}->{}", begin, end)
                });

                begin = nums[i];
                end = nums[i];
            }
        }

        result.push(if begin == end {
            format!("{}", begin)
        } else {
            format!("{}->{}", begin, end)
        });

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_228() {
    }
}
