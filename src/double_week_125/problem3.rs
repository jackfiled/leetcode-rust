pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len();
        let mut graph = vec![vec![];n];
        let signal_speed = signal_speed as i64;

        for edge in &edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let weight = edge[2] as i64;

            graph[x].push((y, weight));
            graph[y].push((x, weight));
        }

        let mut connection = vec![(0,0);n];
        for next in &graph[0] {

        }        

        let mut result = Vec::with_capacity(n);

        result
    }

    fn dfs(graph: &Vec<Vec<(usize, i64)>>, conection: &mut Vec<(i64, usize)>, now: usize, pre: usize, distance: i64, start: usize) {
        for next in &graph[now] {
            if next.0 == pre {
                continue;
            }

            conection[next.0] = (distance + next.1, start);

            Solution::dfs(graph, conection, next.0, now, distance + next.1, start);
        }
    }

    fn tree_dp(graph: &Vec<Vec<(usize, i64)>>, conection: &mut Vec<(i64, usize)>, result: &mut Vec<i32>, 
        now: usize, pre: usize, signal_speed: i64) {
        let mut count = HashMap::new();

        for node in conection {
            if node.0 % signal_speed == 0 {
                let entry = count.entry(node.1).or_insert(0);

                *entry += 1;
            }
        }

        let count: Vec<&i32> = count.values().collect();
        let mut ans = 0;

        for i in 0..(count.len() - 1) {
            for j in (i + 1)..count.len() {
                ans += count[i] * count[j];
            }
        }

        result[now] = ans;

        for next in &graph[now] {
            if next.0 == pre {
                continue;
            }
        }
    }
}