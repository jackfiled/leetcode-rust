/**
 * [2306] Naming a Company
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        use std::collections::{HashSet, HashMap};
        
        let mut map = HashMap::new();
        
        for idea in ideas.iter() {
            let entry = map.entry(&idea[..1]).or_insert(HashSet::new());
            entry.insert(&idea[1..]);
        }
        
        let mut result = 0;
        let values: Vec<HashSet<&str>> = map.into_iter().map(|p| p.1).collect();
        
        for i in 0..values.len() {
            for j in i + 1..values.len() {
                let intersect = values[i].intersection(&values[j]).count();
                
                result += (values[i].len() - intersect) as i64 * (values[j].len() - intersect) as i64
            }
        }
        
        result * 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2306() {
        assert_eq!(6, Solution::distinct_names(vec_string!("coffee", "donuts", "time", "toffee")));
        assert_eq!(0, Solution::distinct_names(vec_string!("lack", "back")));
    }
}
