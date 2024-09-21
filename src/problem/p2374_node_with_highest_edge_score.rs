
/**
 * [2374] Node With Highest Edge Score
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::cmp::Ordering;

        let mut map = HashMap::new();

        for (i, &v) in edges.iter().enumerate() {
            let entry = map.entry(v).or_insert(0);
            *entry += i;
        }
        
        *map.iter().max_by(|a, b| {
            match a.1.cmp(b.1) {
                Ordering::Less => { Ordering::Less }
                Ordering::Equal => { b.0.cmp(a.0) }
                Ordering::Greater => { Ordering::Greater }
            }
        }).unwrap().0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2374() {
        assert_eq!(7, Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]));
        assert_eq!(0, Solution::edge_score(vec![2, 0, 0, 2]));
    }
}
