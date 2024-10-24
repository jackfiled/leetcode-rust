/**
 * [871] Minimum Number of Refueling Stops
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut stations: Vec<(i32, i32)> = stations.iter().map(|v| (v[0], v[1])).collect();
        stations.push((target, 0));

        let mut heap = BinaryHeap::new();
        let mut fuel = start_fuel;
        let mut last_pos = 0;
        let mut count = 0;

        for (p, f) in stations {
            while fuel < p - last_pos {
                if let Some(head) = heap.pop() {
                    fuel += head;
                    count += 1;
                } else {
                    return -1;
                }
            }

            fuel -= p - last_pos;
            last_pos = p;
            heap.push(f);
        }

        if fuel >= 0 {
            count
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_871() {
        assert_eq!(0, Solution::min_refuel_stops(1, 1, vec![]));
        assert_eq!(-1, Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]));
        assert_eq!(
            2,
            Solution::min_refuel_stops(
                100,
                10,
                vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
            )
        );
    }
}
