/**
 * [399] Evaluate Division
 */
pub struct Solution {}


// submission codes start here
use std::collections::HashMap;

struct UnionFind {
    parents : Vec<usize>,
    weights : Vec<f64>
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parents = Vec::with_capacity(n);
        let mut weights = Vec::with_capacity(n);

        for i in 0..n {
            parents.push(i);
            weights.push(1f64);
        }

        UnionFind {
            parents,
            weights
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }

        let origin = self.parents[x];
        self.parents[x] = self.find(origin);
        self.weights[x] *= self.weights[origin];
        self.parents[x]
    }

    fn merge(&mut self, x: usize, y: usize, value: f64) {
        let (x_root, y_root) = (self.find(x), self.find(y));

        if x_root == y_root {
            return;
        }

        self.parents[x_root] = y_root;
        self.weights[x_root] = value * self.weights[y] / self.weights[x];
    }

    fn is_connected(&mut self, x: usize, y: usize) -> f64 {
        let (x_root, y_root) = (self.find(x), self.find(y));

        if x_root == y_root {
            return self.weights[x] / self.weights[y];
        }

        -1f64
    }
}

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let count = equations.len();
        let mut union_find = UnionFind::new(count * 2);
        let mut id_map = HashMap::with_capacity(count * 2);
        let mut id = 0;

        for (i, equation) in equations.iter().enumerate() {
            let (x, y) = (&equation[0], &equation[1]);
            
            let x_id = *id_map.entry(x).or_insert_with(|| {
                let inserted_id = id;
                id += 1;
                inserted_id
            });

            let y_id = *id_map.entry(y).or_insert_with(|| {
                let inserted_id = id;
                id += 1;
                inserted_id
            });

            union_find.merge(x_id, y_id, values[i]);
        }

        let mut result = Vec::with_capacity(queries.len());
        for query in &queries {
            let (x, y) = (&query[0], &query[1]);

            let (x_id, y_id) = (id_map.get(x), id_map.get(y));

            if x_id.is_none() || y_id.is_none() {
                result.push(-1f64);
            } else {
                result.push(union_find.is_connected(*x_id.unwrap(),*y_id.unwrap()));
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
    fn test_399() {
        let mut map = HashMap::new();
        let x = "a".to_owned();
        let y = "a".to_owned();

        map.entry(&x).or_insert(1);

        assert_eq!(map.get(&y), Some(&1));
    }
}
