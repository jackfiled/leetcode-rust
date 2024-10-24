/**
 * [447] Number of Boomerangs
 *
 * You are given n points in the plane that are all distinct, where points[i] = [xi, yi]. A boomerang is a tuple of points (i, j, k) such that the distance between i and j equals the distance between i and k (the order of the tuple matters).
 * Return the number of boomerangs.
 *  
 * <strong class="example">Example 1:
 *
 * Input: points = [[0,0],[1,0],[2,0]]
 * Output: 2
 * Explanation: The two boomerangs are [[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]].
 *
 * <strong class="example">Example 2:
 *
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 2
 *
 * <strong class="example">Example 3:
 *
 * Input: points = [[1,1]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == points.length
 * 	1 <= n <= 500
 * 	points[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/number-of-boomerangs/
// discuss: https://leetcode.cn/problems/number-of-boomerangs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for p in &points {
            let mut values = HashMap::new();

            for q in &points {
                let dis = (p[0] - q[0]).pow(2) + (p[1] - q[1]).pow(2);

                let c = values.entry(dis).or_insert(0);
                *c += 1;
            }

            for (_, v) in &values {
                // 实际上并不需要，因为最小值为1
                // 计算的结果为0
                if *v >= 2 {
                    // result += A_m^2
                    result += (*v) * (*v - 1);
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_447() {}
}
