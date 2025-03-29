/**
 * [2360] Longest Cycle in a Graph
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut labels = vec![0; n];

        let mut current_label = 0;
        let mut result = -1;

        for i in 0..n {
            if labels[i] != 0 {
                continue;
            }

            let mut pos = i as i32;
            let start_label = current_label;

            while pos != -1 {
                current_label += 1;
                // 遇到已经遍历过的节点
                let real_pos = pos as usize;
                if labels[real_pos] != 0 {
                    if labels[real_pos] > start_label {
                        result = result.max(current_label - labels[real_pos]);
                    }
                    break;
                }

                labels[real_pos] = current_label;
                pos = edges[real_pos];
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
    fn test_2360() {
        assert_eq!(3, Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
        assert_eq!(-1, Solution::longest_cycle(vec![2, -1, 3, 1]));
    }
}
