/**
 * [1450] Number of Students Doing Homework at a Given Time
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let start_pos = start_time.iter().enumerate().filter_map(|(pos, &value)| {
            if value <= query_time {
                Some(pos)
            } else {
                None
            }
        });

        let pos = start_pos.filter_map(|pos| {
            if end_time[pos] >= query_time {
                Some(pos)
            } else {
                None
            }
        });

        pos.count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1450() {
        assert_eq!(1, Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4));
        assert_eq!(1, Solution::busy_student(vec![4], vec![4], 4));
    }
}
