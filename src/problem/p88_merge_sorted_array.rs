/**
 * [88] Merge Sorted Array
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut result = Vec::with_capacity(m + n);

        let (mut i, mut j) = (0, 0);

        while i < m || j < n {
            if i == m {
                while j < n {
                    result.push(nums2[j]);
                    j += 1;
                }

                break;
            }

            if j == n {
                while i < m {
                    result.push(nums1[i]);
                    i += 1;
                }

                break;
            }


            if nums1[i] < nums2[j] {
                result.push(nums1[i]);
                i += 1;
            } else {
                result.push(nums2[j]);
                j += 1;
            }
        }

        for (index, &value) in result.iter().enumerate() {
            nums1[index] = value;
        }    
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }
}
