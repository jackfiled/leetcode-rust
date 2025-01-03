/**
 * [731] My Calendar II
 */
pub struct Solution {}

// submission codes start here
use std::collections::BTreeMap;

struct MyCalendarTwo {
    map: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        let mut max_book = 0;

        let start = self.map.entry(start_time).or_insert(0);
        *start += 1;
        let end = self.map.entry(end_time).or_insert(0);
        *end -= 1;

        for (_, &freq) in self.map.iter() {
            max_book += freq;

            if max_book > 2 {
                let start = self.map.entry(start_time).or_insert(0);
                *start -= 1;
                let end = self.map.entry(end_time).or_insert(0);
                *end += 1;
                return false;
            }
        }

        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(startTime, endTime);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_731() {
        let mut calendar = MyCalendarTwo::new();

        assert!(calendar.book(24, 40));
        assert!(calendar.book(43, 50));
        assert!(calendar.book(27, 43));
        assert!(calendar.book(5, 21));
        assert!(!calendar.book(30, 40));
        assert!(!calendar.book(14, 29));
        assert!(calendar.book(3, 19));
        assert!(!calendar.book(3, 14));
        assert!(!calendar.book(25, 39));
        assert!(!calendar.book(6, 19));

        let mut calendar = MyCalendarTwo::new();

        assert!(calendar.book(10, 20));
        assert!(calendar.book(50, 60));
        assert!(calendar.book(10, 40));
        assert!(!calendar.book(5, 15));
        assert!(calendar.book(5, 10));
        assert!(calendar.book(25, 55));
    }
}
