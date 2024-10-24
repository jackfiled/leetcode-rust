/**
 * [815] Bus Routes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        use std::collections::{HashMap, VecDeque};

        if source == target {
            return 0;
        }

        let n = routes.len();
        // 存储经过某一站的公交线路
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        // 存储不同公交线路之前的联通情况
        let mut edge = vec![vec![false; n]; n];

        for (bus, stations) in routes.iter().enumerate() {
            for &station in stations {
                if let Some(connected_stations) = map.get(&station) {
                    for &connected_station in connected_stations {
                        edge[connected_station][bus] = true;
                        edge[bus][connected_station] = true;
                    }
                }

                let entry = map.entry(station).or_insert(vec![]);
                entry.push(bus);
            }
        }

        let mut distances = vec![-1; n];
        let mut queue = VecDeque::new();

        if let Some(source_buses) = map.get(&source) {
            for &bus in source_buses {
                distances[bus] = 1;
                queue.push_back(bus);
            }
        } else {
            // 没有任何公交车经过起始站点
            return -1;
        }

        while !queue.is_empty() {
            let now = queue.pop_front().unwrap();

            for next in 0..n {
                if edge[now][next] && distances[next] == -1 {
                    distances[next] = distances[now] + 1;
                    queue.push_back(next);
                }
            }
        }

        let mut result = i32::MAX;

        if let Some(target_buses) = map.get(&target) {
            for &bus in target_buses {
                result = result.min(distances[bus]);
            }
        } else {
            // 没有任何公交车经过终点站点
            return -1;
        }

        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_815() {
        assert_eq!(
            2,
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6)
        );
    }
}
