/**
 * [433] Minimum Genetic Mutation
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut set = HashSet::with_capacity(bank.len());
        let length = start_gene.len();
        let keys = vec!['A', 'T', 'C', 'G'];

        for i in bank {
            set.insert(i);
        }

        let mut result = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start_gene);

        while !queue.is_empty() {
            let count = queue.len();

            for _ in 0..count {
                let gene = queue.pop_front().unwrap();
                if gene == end_gene {
                    return result;
                }

                let gene: Vec<char> = gene.chars().collect();

                for i in 0..length {
                    for &k in &keys {
                        if gene[i] == k {
                            continue;
                        }

                        let mut next_gene = gene.clone();
                        next_gene[i] = k;
                        let next_gene: String = next_gene.iter().collect();

                        if !set.contains(&next_gene) {
                            continue;
                        }

                        if visited.contains(&next_gene) {
                            continue;
                        }
                        visited.insert(next_gene.clone());

                        queue.push_back(next_gene);
                    }
                }
            }

            result += 1;
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_433() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AAACGGTA".to_owned(),
                vec![
                    "AACCGGTA".to_owned(),
                    "AACCGCTA".to_owned(),
                    "AAACGGTA".to_owned()
                ]
            ),
            2
        );
    }
}
