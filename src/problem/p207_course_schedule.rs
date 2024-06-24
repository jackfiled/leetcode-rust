/**
 * [207] Course Schedule
 */
pub struct Solution {}


// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut edges = vec![vec![];num_courses];
        let mut in_degs = vec![0;num_courses];

        for edge in prerequisites {
            edges[edge[1] as usize].push(edge[0] as usize);
            in_degs[edge[0] as usize] += 1;
        }

        let mut queue = VecDeque::new();

        for i in 0..num_courses {
            if in_degs[i] == 0 {
                queue.push_back(i);
            }
        } 

        let mut visited = 0;
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap(); 
            visited += 1;

            for next_node in &edges[node] {
                in_degs[*next_node] -= 1;
                
                if in_degs[*next_node] == 0 {
                    queue.push_back(*next_node);
                }
            }            
        }

        visited == num_courses
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
    }
}
