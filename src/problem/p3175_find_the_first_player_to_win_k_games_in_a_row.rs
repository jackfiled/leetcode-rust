/**
 * [3175] Find The First Player to win K Games in a Row
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let n = skills.len();
        let k = k as usize;
        let mut pos = 0;

        for i in 1..n {
            if skills[i] > skills[pos] {
                pos = i;
            }
            if pos == 0 {
                if i - pos >= k {
                    break;
                }
            } else {
                if i - pos >= k - 1 {
                    break;
                }
            }
        }

        pos as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3175() {
        assert_eq!(1, Solution::find_winning_player(vec![1, 6, 17], 1));
        assert_eq!(2, Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2));
        assert_eq!(1, Solution::find_winning_player(vec![2, 5, 4], 3));
    }
}
