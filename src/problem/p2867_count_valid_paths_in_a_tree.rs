/**
 * [2867] Count Valid Paths in a Tree
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    fn dfs(graph: &Vec<Vec<usize>>, primes: &Vec<bool>, seen: &mut Vec<usize>, 
        i: usize, pre: usize) {
        seen.push(i);

        for next in &graph[i] {
            let next = *next;
            if next != pre && !primes[next] {
                Solution::dfs(graph, primes, seen, next, i);
            } 
        }
    }

    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut primes = vec![true;n + 1];
        primes[1] = false;
        for i in 2..=n {
            if primes[i] {
                if i * i > n {
                    continue;
                }
    
                for j in (i * i..=n).step_by(i) {
                    primes[j] = false;
                }
            }
        }
        primes = dbg!(primes);

        let mut graph = vec![vec![];n + 1];
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            graph[x].push(y);
            graph[y].push(x);
        }

        let mut count = vec![0; n + 1];
        let mut result = 0;

        for root in 1..=n {
            if !primes[root] {
                continue;
            }
            
            let mut current = 0;
            for i in &graph[root] {
                let i = *i;
                if (primes[i]) {
                    continue;
                }
                
                if count[i] == 0 {
                    let mut seen = Vec::new();
                    Solution::dfs(&graph, &primes, &mut seen, i, 0);
                    for k in &seen {
                        count[*k] = seen.len() as i64; 
                    }
                }

                result += count[i] * current;
                current += count[i];
            }

            result += current;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2867() {
        assert_eq!(6, Solution::count_paths(6, 
            vec![vec![1,2],vec![1,3],vec![2,4],vec![3,5],vec![3,6]]));
    }
}
