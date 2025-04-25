/**
 * [2080] Range Frequency Queries
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

struct RangeFreqQuery {
    frequency_map: HashMap<i32, Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut frequency_map = HashMap::new();

        for (i, v) in arr.into_iter().enumerate() {
            let entry = frequency_map.entry(v).or_insert(vec![]);
            entry.push(i);
        }

        Self { frequency_map }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);

        if let Some(index) = self.frequency_map.get(&value) {
            let l = index.binary_search(&left);
            let r = index.binary_search(&right);

            let result = match l {
                Ok(l_ok) => match r {
                    Ok(r_ok) => r_ok - l_ok + 1,
                    Err(r_err) => r_err - l_ok,
                },
                Err(l_err) => match r {
                    Ok(r_ok) => r_ok - l_err + 1,
                    Err(r_err) => r_err - l_err,
                },
            };

            result as i32
        } else {
            0
        }
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2080() {
        let query = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);

        assert_eq!(1, query.query(1, 2, 4));
        assert_eq!(2, query.query(0, 11, 33));

        let query = RangeFreqQuery::new(vec![2, 2, 1, 2, 2]);

        // [2]
        assert_eq!(1, query.query(2, 4, 1));
        assert_eq!(1, query.query(1, 3, 1));
        assert_eq!(1, query.query(0, 2, 1));

        let query = RangeFreqQuery::new(vec![1, 1, 1, 2, 2]);

        // [0, 1, 2]
        assert_eq!(1, query.query(2, 2, 1));
    }
}
