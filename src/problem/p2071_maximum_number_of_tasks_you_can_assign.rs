/**
 * [2071] Maximum Number of Tasks You Can Assign
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        let (n, m) = (tasks.len(), workers.len());

        tasks.sort_unstable();
        workers.sort_unstable();

        let check = |middle: usize| -> bool {
            // 注意middle的溢出问题
            if middle < 1 {
                return true;
            }

            let mut p = pills;
            let mut worker_queue = VecDeque::new();
            let mut ptr = m - 1;

            for i in (0..=middle - 1).rev() {
                while ptr >= m - middle && workers[ptr] + strength >= tasks[i] {
                    worker_queue.push_front(workers[ptr]);

                    if ptr == 0 {
                        break;
                    }
                    ptr -= 1;
                }

                if worker_queue.is_empty() {
                    return false;
                }

                let last = worker_queue.back().unwrap();

                if *last >= tasks[i] {
                    worker_queue.pop_back();
                } else {
                    if p == 0 {
                        return false;
                    }

                    p -= 1;
                    worker_queue.pop_front();
                }
            }

            return true;
        };

        let (mut left, mut right) = (1, m.min(n));
        let mut result = 0;

        while left <= right {
            let middle = (left + right) / 2;
            if check(middle) {
                result = middle;
                left = middle + 1;
            } else {
                right = middle - 1;
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
    fn test_2071() {
        assert_eq!(
            1,
            Solution::max_task_assign(vec![35], vec![83, 20, 4, 66], 3, 41)
        );
        assert_eq!(
            3,
            Solution::max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1)
        );
        assert_eq!(
            1,
            Solution::max_task_assign(vec![5, 4], vec![0, 0, 0], 1, 5)
        );
        assert_eq!(
            2,
            Solution::max_task_assign(vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10)
        );
        assert_eq!(
            3,
            Solution::max_task_assign(vec![5, 9, 8, 5, 9], vec![1, 6, 4, 2, 6], 1, 5)
        );
    }
}
