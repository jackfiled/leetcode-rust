/**
 * [295] Find Median from Data Stream
 */
pub struct Solution {}

// submission codes start here
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new()
        }
    }
    
    fn add_num(&mut self, num: i32) {
        // 保证如下的状态
        // left.len = right.len + 1
        // left.len = right.len
        
        if self.left.len() > self.right.len() {
            // 此时左边必然有数
            let left_peek = *self.left.peek().unwrap();
            
            if num >= left_peek {
                self.right.push(Reverse(num))
            } else { 
                self.right.push(Reverse(left_peek));
                self.left.pop();
                self.left.push(num);
            }
        } else {
            // 判空
            if self.left.is_empty() {
                self.left.push(num);
            } else { 
                // 此时必然两端都有数
                let right_peek = self.right.peek().unwrap().0;
                
                if num <= right_peek {
                    self.left.push(num);
                } else { 
                    self.left.push(right_peek);
                    self.right.pop();
                    self.right.push(Reverse(num));
                }
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        return if self.left.len() == self.right.len() {
            (*self.left.peek().unwrap() as f64 + self.right.peek().unwrap().0 as f64) / 2f64
        } else { 
            *self.left.peek().unwrap() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_295() {
        let mut finder = MedianFinder::new();
        
        finder.add_num(1);
        finder.add_num(2);
        
        assert_eq!(1.5, finder.find_median());
        
        finder.add_num(3);
        finder = dbg!(finder);
        
        assert_eq!(2f64, finder.find_median());
    }
}
