pub struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Worker {
    time: i64,
    base_time: i64,
    used_time: i64,
    count: i64,
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.used_time + other.time).cmp(&(self.used_time + self.time)) 
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut worker_times: BinaryHeap<Worker> = worker_times.iter().enumerate().map(|(i, v)| Worker {
            time: *v as i64,
            base_time: *v as i64,
            used_time: 0,
            count: 1,
        }).collect();
        
        for _ in 0..mountain_height {
            let min_worker = worker_times.pop().unwrap();
            
            worker_times.push(Worker {
                base_time: min_worker.base_time,
                count: min_worker.count + 1,
                used_time: min_worker.used_time + min_worker.time,
                time: min_worker.base_time * (min_worker.count + 1)
            });
        }
        
        worker_times.iter().map(|w| w.used_time).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_416_2() {
        assert_eq!(10, Solution::min_number_of_seconds(5, vec![1, 5]));
        assert_eq!(3, Solution::min_number_of_seconds(4, vec![2,1,1]));
        assert_eq!(12, Solution::min_number_of_seconds(10, vec![3,2,2,4]));
        assert_eq!(15, Solution::min_number_of_seconds(5, vec![1]));
    }
}
