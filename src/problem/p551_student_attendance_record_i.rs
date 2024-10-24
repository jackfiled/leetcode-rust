/**
 * [551] Student Attendance Record I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_record(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut late_count = 0;
        let mut absent_count = 0;

        for c in s {
            match c {
                'A' => {
                    absent_count += 1;
                    late_count = 0;

                    if absent_count >= 2 {
                        return false;
                    }
                }
                'L' => {
                    late_count += 1;

                    if late_count >= 3 {
                        return false;
                    }
                }
                _ => {
                    late_count = 0;
                }
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_551() {}
}
