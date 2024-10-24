/**
 * [1436] Destination City
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;

        let mut out_map = HashMap::new();

        for path in paths.iter() {
            let start_entry = out_map.entry(&path[0]).or_insert(0);
            *start_entry += 1;

            out_map.entry(&path[1]).or_insert(0);
        }

        out_map
            .iter()
            .filter(|(_, &v)| v == 0)
            .map(|(k, _)| k.to_string())
            .next()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1436() {
        assert_eq!(
            "A".to_owned(),
            Solution::dest_city(vec![
                vec_string!("B", "C"),
                vec_string!("D", "B"),
                vec_string!("C", "A")
            ])
        );
    }
}
