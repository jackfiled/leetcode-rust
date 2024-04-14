/**
 * [380] Insert Delete GetRandom O(1)
 */
pub struct Solution {}


// submission codes start here
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;

struct RandomizedSet {
    array: Vec<i32>,
    pos_table: HashMap<i32, usize>,
    pos_now: usize,
    rng: ThreadRng
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet {
            array: vec![0;200_001],
            pos_table: HashMap::new(),
            pos_now: 0,
            rng: rand::thread_rng()
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.pos_table.contains_key(&val) {
            return false;
        }

        self.array[self.pos_now] = val;
        self.pos_table.insert(val, self.pos_now);
        self.pos_now += 1;
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        return match self.pos_table.get(&val) {
            Some(&pos) => {
                let last = self.array[self.pos_now - 1];
                self.pos_table.insert(last, pos);
                self.array[pos] = last;
                self.pos_now -= 1; 
                self.pos_table.remove(&val);

                true
            },
            None => false
        }
    }
    
    fn get_random(&mut self) -> i32 {
        let pos = self.rng.gen_range(0..self.pos_now);
        self.array[pos]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_380() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(0));
        assert!(set.insert(1));
        assert!(set.remove(0));
        assert!(set.insert(2));
        assert!(set.remove(1));
        assert_eq!(2, set.get_random());
    }
}
