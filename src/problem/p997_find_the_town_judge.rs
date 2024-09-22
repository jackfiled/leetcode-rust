use regex::escape;

/**
 * [997] Find the Town Judge
 */
pub struct Solution {}
    

// submission codes start here

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = n as usize;

        let mut trust_map = HashMap::with_capacity(n);
        let mut trusted_map = HashMap::with_capacity(n);

        for i in 1..=n {
            trust_map.insert(i, 0);
        }

        for i in trust {
            let (a, b) = (i[0] as usize, i[1] as usize);

            let entry = trust_map.entry(a).or_insert(0);
            *entry += 1;

            let entry = trusted_map.entry(b).or_insert(vec![]);
            entry.push(a);
        }

        for man in trust_map.iter().filter(|p| *p.1 == 0) {
            if let Some(people) = trusted_map.get(man.0) {
                if people.len() == n - 1 && !people.contains(man.0) {
                    return *man.0 as i32;
                }
            } else {
                if n == 1 {
                    return *man.0 as i32;
                }
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_997() {
        assert_eq!(2, Solution::find_judge(2, vec![vec![1, 2]]));
        assert_eq!(3, Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]));
        assert_eq!(1, Solution::find_judge(1, vec![]));
        assert_eq!(-1, Solution::find_judge(2, vec![]));
    }
}
