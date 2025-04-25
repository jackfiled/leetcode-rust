/**
 * [2502] Design Memory Allocator
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

struct Allocator {
    memory: Vec<bool>,
    id_map: HashMap<i32, Vec<(usize, usize)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            memory: vec![false; n],
            id_map: HashMap::new(),
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let mut start = 0;

        while start < self.memory.len() {
            let mut pos = start;
            while pos < self.memory.len() && !self.memory[pos] {
                if pos - start + 1 == size {
                    for i in start..=pos {
                        self.memory[i] = true;
                    }

                    let entry = self.id_map.entry(m_id).or_insert(vec![]);
                    entry.push((start, pos));
                    return start as i32;
                }

                pos += 1;
            }

            // 到达这里只能说明找到的空间不足
            start = pos + 1;
        }

        -1
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        if let Some(array) = self.id_map.get(&m_id) {
            let mut length = 0;

            for &(start, end) in array.iter() {
                for i in start..=end {
                    self.memory[i] = false;
                }
                length += (end - start + 1) as i32;
            }

            self.id_map.remove(&m_id);
            length
        } else {
            0
        }
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free_memory(mID);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2502() {
        let mut allocator = Allocator::new(10);

        assert_eq!(0, allocator.allocate(1, 1));
        assert_eq!(1, allocator.allocate(1, 2));
        assert_eq!(2, allocator.allocate(1, 3));

        assert_eq!(1, allocator.free_memory(2));

        assert_eq!(3, allocator.allocate(3, 4));
        assert_eq!(1, allocator.allocate(1, 1));
        assert_eq!(6, allocator.allocate(1, 1));

        assert_eq!(3, allocator.free_memory(1));

        assert_eq!(-1, allocator.allocate(10, 2));
        assert_eq!(0, allocator.free_memory(7));
    }
}
