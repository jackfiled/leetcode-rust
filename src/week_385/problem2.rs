pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut set = HashSet::new();

        for num in arr1 {
            let mut num = num;

            while num != 0 {
                set.insert(num);
                num = num / 10;
            }
        }

        for num in arr2 {
            let mut len = 0;
            let mut i = num;
            while i != 0 {
                len += 1;
                i = i / 10;
            }
            let mut i = num;
            while i != 0 {
                if set.contains(&i) {
                    result = result.max(len);
                }
                len -= 1;
                i = i / 10;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_385_2() {}
}
