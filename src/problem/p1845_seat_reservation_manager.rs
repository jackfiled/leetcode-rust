/**
 * [1845] Seat Reservation Manager
 */
pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct SeatManager {
    seats: BinaryHeap<Reverse<i32>>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        let mut heap = BinaryHeap::with_capacity(n as usize);
        
        for i in 1..=n {
            heap.push(Reverse(i));
        }
        
        Self {
            seats: heap
        }
    }
    
    fn reserve(&mut self) -> i32 {
        let head = self.seats.pop();
        head.unwrap().0
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(Reverse(seat_number));
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1845() {
        let mut manager = SeatManager::new(5);
        
        assert_eq!(1, manager.reserve());
    }
}
