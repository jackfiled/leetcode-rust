/**
 * [502] IPO
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        
        let mut array = vec![];
        for (&p, &c) in profits.iter().zip(capital.iter()) {
            array.push((p, c));
        }
        array.sort_by(|a, b| a.1.cmp(&b.1));
        
        let mut result = w;
        let mut current = 0;
        let mut heap = BinaryHeap::new();
        let n = array.len();
        
        for _ in 0..k {
            while current < n && array[current].1 <= result {
                heap.push(array[current].0);
                current += 1;
            }
            
            if let Some(&head) = heap.peek() {
                result += head;
                heap.pop();
            } else {
                break;
            }
        }
        
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_502() {
    }
}
