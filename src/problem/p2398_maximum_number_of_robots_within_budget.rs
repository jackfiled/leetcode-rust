/**
 * [2398] Maximum Number of Robots Within Budget
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        use std::collections::VecDeque;

        let n = charge_times.len();
        let mut result = 0;
        let mut running_costs_sum = 0i64;
        let mut queue = VecDeque::with_capacity(n);

        // 最左侧的机器人
        let mut left = 0;

        for i in 0..n {
            running_costs_sum += running_costs[i] as i64;

            // 维持单调队列
            while let Some(&pos) = queue.back() {
                if charge_times[pos] <= charge_times[i] {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            queue.push_back(i);

            // 判断当前left开始的机器人是否会导致超过预算
            while left <= i && (i - left + 1) as i64 * running_costs_sum + charge_times[*queue.front().unwrap()] as i64 > budget {
                if let Some(&front) = queue.front() {
                    if front == left {
                        queue.pop_front();
                    }
                }

                running_costs_sum -= running_costs[left] as i64;
                left += 1;
            }
            if i >= left {
                result = result.max(i - left + 1);
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2398() {
        assert_eq!(3, Solution::maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25));
        assert_eq!(0, Solution::maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19));
    }
}
