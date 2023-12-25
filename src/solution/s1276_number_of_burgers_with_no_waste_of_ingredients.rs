/**
 * [1276] Number of Burgers with No Waste of Ingredients
 *
 * Given two integers tomatoSlices and cheeseSlices. The ingredients of different burgers are as follows:
 * 
 * 	Jumbo Burger: 4 tomato slices and 1 cheese slice.
 * 	Small Burger: 2 Tomato slices and 1 cheese slice.
 * 
 * Return [total_jumbo, total_small] so that the number of remaining tomatoSlices equal to 0 and the number of remaining cheeseSlices equal to 0. If it is not possible to make the remaining tomatoSlices and cheeseSlices equal to 0 return [].
 *  
 * <strong class="example">Example 1:
 * 
 * Input: tomatoSlices = 16, cheeseSlices = 7
 * Output: [1,6]
 * Explantion: To make one jumbo burger and 6 small burgers we need 4*1 + 2*6 = 16 tomato and 1 + 6 = 7 cheese.
 * There will be no remaining ingredients.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: tomatoSlices = 17, cheeseSlices = 4
 * Output: []
 * Explantion: There will be no way to use all ingredients to make small and jumbo burgers.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: tomatoSlices = 4, cheeseSlices = 17
 * Output: []
 * Explantion: Making 1 jumbo burger there will be 16 cheese remaining and making 2 small burgers there will be 15 cheese remaining.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= tomatoSlices, cheeseSlices <= 10^7
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients/
// discuss: https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let (mut tomato, mut cheese) = (tomato_slices as f32,
                                    cheese_slices as f32);
        if tomato == 0f32 && cheese == 0f32 {
            return vec![0, 0]
        }

        if tomato == 0f32 ||
            tomato / cheese < 2f32 || tomato / cheese > 4f32 {
            return vec![]
        }

        let mut big_count = 0;

        while tomato / cheese != 2f32 {
            tomato -= 4f32;
            cheese -= 1f32;
            big_count += 1;

            if tomato == 0f32 && cheese == 0f32 {
                break;
            }

            if tomato * cheese == 0f32 {
                return vec![]
            }
        }
        
        vec![big_count, cheese as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1276() {
        let empty_array : Vec<i32> = Vec::new();
        assert_eq!(vec![1, 6], Solution::num_of_burgers(16, 7));
        assert_eq!(empty_array, Solution::num_of_burgers(17, 4));
        assert_eq!(empty_array, Solution::num_of_burgers(4, 17));
        assert_eq!(vec![0, 0], Solution::num_of_burgers(0, 0));
        assert_eq!(vec![0, 1], Solution::num_of_burgers(2, 1));
    }
}
