/**
 * [3159] Find Occurrences of an Element in an Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let x_pos: Vec<usize> = nums
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v == x { Some(i) } else { None })
            .collect();

        let result = queries
            .into_iter()
            .map(|x| {
                let x = x as usize - 1;
                if x >= x_pos.len() {
                    -1
                } else {
                    x_pos[x] as i32
                }
            })
            .collect();

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3159() {
        assert_eq!(
            vec![0, -1, 2, -1],
            Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1)
        );
        assert_eq!(
            vec![-1],
            Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5)
        );
    }
}
