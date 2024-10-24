/**
 * [77] Combinations
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut now = Vec::with_capacity(k as usize);

        Self::search(n, k, 1, &mut now, &mut result);

        result
    }

    fn search(n: i32, k: i32, x: i32, now: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if now.len() as i32 + (n - x + 1) < k {
            return;
        }

        if now.len() as i32 >= k {
            result.push(now.clone());
            return;
        }

        now.push(x);
        Self::search(n, k, x + 1, now, result);
        now.pop();
        Self::search(n, k, x + 1, now, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_77() {}
}
