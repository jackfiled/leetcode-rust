/**
 * [74] Search a 2D Matrix
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        // 左闭右开区间
        let (mut lower, mut upper) = (0, m);

        while lower < upper {
            let middle = (upper - lower) / 2 + lower;
            
            // WTF?
            if lower == middle {
                break;
            }

            if target < matrix[middle][0] {
                upper = middle;
            } else {
                lower = middle;
            }
        }

        let (mut left, mut right) = (0, n);

        while left < right {
            let middle = (right - left) / 2 + left;

            if target > matrix[lower][middle] {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        if left < n {
            matrix[lower][left] == target
        } else { 
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_74() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        
        assert!(Solution::search_matrix(matrix, 3));
        
        let matrix = vec![vec![1]];
        
        assert!(!Solution::search_matrix(matrix, 2));
    }
}
