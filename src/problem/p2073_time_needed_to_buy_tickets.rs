/**
 * [2073] Time Needed to Buy Tickets
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let k = k as usize;

        let mut queue = VecDeque::with_capacity(tickets.len());
        for (i, &v) in tickets.iter().enumerate() {
            queue.push_back(if i == k { (v, true) } else { (v, false) });
        }

        let mut second = 0;

        loop {
            let head = queue.pop_front().unwrap();
            second += 1;

            if head.0 == 1 {
                // 卖完了
                if head.1 {
                    break;
                }
            } else {
                queue.push_back((head.0 - 1, head.1));
            }
        }

        second
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2073() {
        assert_eq!(6, Solution::time_required_to_buy(vec![2, 3, 2], 2));
        assert_eq!(8, Solution::time_required_to_buy(vec![5, 1, 1, 1], 0));
    }
}
