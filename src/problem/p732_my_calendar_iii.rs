/**
 * [732] My Calendar III
 */
pub struct Solution {}

// submission codes start here
use std::collections::BTreeMap;

struct MyCalendarThree {
    map: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        let mut max_book = 0;
        let mut result = 0;

        let start = self.map.entry(start_time).or_insert(0);
        *start += 1;
        let end = self.map.entry(end_time).or_insert(0);
        *end -= 1;

        for (_, &freq) in self.map.iter() {
            max_book += freq;
            result = result.max(max_book);
        }

        result
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_732() {
        let mut calendar = MyCalendarThree::new();

        assert_eq!(1, calendar.book(10, 20));
        assert_eq!(1, calendar.book(50, 60));
        assert_eq!(2, calendar.book(10, 40));
        assert_eq!(3, calendar.book(5, 15));
        assert_eq!(3, calendar.book(5, 10));
        assert_eq!(3, calendar.book(25, 55));
    }
}
