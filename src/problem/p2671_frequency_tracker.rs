/**
 * [2671] Frequency Tracker
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashMap, HashSet};

struct FrequencyTracker {
    number_map: HashMap<i32, i32>,
    frequency_map: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            number_map: HashMap::new(),
            frequency_map: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let mut entry = self.number_map.entry(number).or_insert(0);

        if *entry != 0 {
            if let Some(set) = self.frequency_map.get_mut(&entry) {
                set.remove(&number);
            }
        }

        *entry += 1;
        let set = self.frequency_map.entry(*entry).or_insert(HashSet::new());
        set.insert(number);
    }

    fn delete_one(&mut self, number: i32) {
        if let Some(entry) = self.number_map.get_mut(&number) {
            if *entry == 0 {
                return;
            }

            if let Some(set) = self.frequency_map.get_mut(&entry) {
                set.remove(&number);
            }

            *entry -= 1;
            let set = self.frequency_map.entry(*entry).or_insert(HashSet::new());
            set.insert(number);
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        let set = self.frequency_map.get(&frequency);

        match set {
            Some(set) => set.len() != 0,
            None => false,
        }
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2671() {
        let mut tracker = FrequencyTracker::new();

        tracker.add(10);
        tracker.delete_one(5);
        assert!(tracker.has_frequency(1));
        tracker.delete_one(10);
        tracker.delete_one(9);
        tracker.delete_one(10);
        assert!(!tracker.has_frequency(1));
        tracker.add(4);
        assert!(tracker.has_frequency(1));
        tracker.delete_one(4);
        assert!(!tracker.has_frequency(1));
        assert!(!tracker.has_frequency(1));
        tracker.add(10);
        assert!(tracker.has_frequency(1));
    }
}
