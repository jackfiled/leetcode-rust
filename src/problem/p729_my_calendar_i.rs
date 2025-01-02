/**
 * [729] My Calendar I
 */
pub struct Solution {}

// submission codes start here
use std::collections::BTreeMap;

struct MyCalendar {
    // (end, start)
    end_time_map: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            end_time_map: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some((&end, &start)) = self.end_time_map.range(start_time + 1..).next() {
            // 左闭右开区间
            if start < end_time {
                return false;
            }
        }

        self.end_time_map.insert(end_time, start_time);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(startTime, endTime);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_729() {
        let mut calendar = MyCalendar::new();

        assert!(calendar.book(10, 20));
        assert!(!calendar.book(15, 25));
        assert!(calendar.book(20, 30));
    }
}
