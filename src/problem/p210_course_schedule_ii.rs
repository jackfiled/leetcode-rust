/**
 * [210] Course Schedule II
 */
pub struct Solution {}


// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num = num_courses as usize;
        let mut edges = vec![vec![];num];
        let mut in_degs = vec![0;num];

        for edge in prerequisites {
            let (x, y) = (edge[1] as usize, edge[0] as usize);

            edges[x].push(y);
            in_degs[y] += 1;
        }

        let mut queue = VecDeque::new();

        for i in 0..num {
            if in_degs[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut result = vec![];

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            result.push(node);

            for &next in &edges[node] {
                in_degs[next] -= 1;

                if in_degs[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if result.len() != num {
            vec![]
        } else {
            result.iter().map(|x| *x as i32).collect()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_210() {
    }
}
