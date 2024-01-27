/**
 * [2861] Maximum Number of Alloys
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    fn check(n: usize, count: i32, budget: i32, composition: &Vec<i32>, stock: &Vec<i32>, cost: &Vec<i32>) -> bool {
        let mut need = 0i64;

        for i in 0..n {
            let number = composition[i] * count - stock[i];

            if number > 0 {
                need += number as i64 * cost[i] as i64;
            }
        }

        need <= budget as i64
    }

    pub fn max_number_of_alloys(n: i32, k: i32, budget: i32,
                                composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut result = 0;
        let (n, k) = (n as usize, k as usize);

        for m in 0..k {
            let mut left = 0;
            let mut right = vec![0;n];

            for i in 0..n {
                right[i] = (stock[i] + budget / cost[i]) / composition[m][i];
            }

            let mut right = *right.iter().max().unwrap();

            while left < right {
                let mid = (left + right) / 2;

                if Solution::check(n, mid, budget, &composition[m], &stock, &cost) {
                    left = mid + 1;
                } else {
                    right = mid;
                }

                dbg!(left, right);
            }

            if !Solution::check(n, left, budget, &composition[m], &stock, &cost) {
                left = left - 1;
            }

            result = result.max(left);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2861() {
        assert_eq!(Solution::max_number_of_alloys(3,2,15,
                                                  vec![vec![1,1,1], vec![1,1,10]],
                                                  vec![0,0,0],
                                                  vec![1,2,3]),
                   2);
    }
}
