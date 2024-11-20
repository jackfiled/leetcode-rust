/**
 * [3244] Shortest Distance After Road Addition Queries II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut roads = vec![None; n];
        for i in 0..n {
            roads[i] = Some(i + 1);
        }

        let mut result = Vec::with_capacity(queries.len());
        let mut distance = n - 1;

        for query in queries {
            let (start, end) = (query[0] as usize, query[1] as usize);
            let mut k = roads[start];
            roads[start] = Some(end);

            while let Some(now) = k {
                if now >= end {
                    break;
                }

                k = roads[now];
                roads[now] = None;
                distance -= 1;
            }

            result.push(distance);
        }

        result.into_iter().map(|x| x as i32).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3244() {
        assert_eq!(
            vec![3, 2, 1],
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]])
        );
        assert_eq!(
            vec![1, 1],
            Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]])
        );
    }
}
