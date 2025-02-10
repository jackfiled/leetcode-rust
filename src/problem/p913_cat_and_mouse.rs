/**
 * [913] Cat and Mouse
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

const MOUSE_TURN: usize = 0;
const CAT_TURN: usize = 1;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Status {
    DRAW,
    MOUSE,
    CAT,
}

struct Gamer {
    n: usize,
    graph: Vec<Vec<usize>>,
    // [i, j, mouse/cat]
    degrees: Vec<Vec<Vec<usize>>>,
    // [i, j, mouse/cat]
    results: Vec<Vec<Vec<Status>>>,
}

impl Gamer {
    fn new(graph: Vec<Vec<i32>>) -> Self {
        let graph = graph
            .into_iter()
            .map(|x| x.into_iter().map(|i| i as usize).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();

        let n = graph.len();
        let mut degrees = vec![vec![vec![0; 2]; n]; n];

        // 初始化每个节点的度
        for i in 0..n {
            for j in 1..n {
                degrees[i][j][MOUSE_TURN] = graph[i].len();
                degrees[i][j][CAT_TURN] = graph[j].len();
            }
        }

        // 猫不能进入洞中
        for &node in graph[0].iter() {
            for i in 0..n {
                degrees[i][node][CAT_TURN] -= 1;
            }
        }

        Self {
            n,
            graph,
            degrees,
            results: vec![vec![vec![Status::DRAW; 2]; n]; n],
        }
    }

    fn search(&mut self) -> i32 {
        // (mouse, cat, turn)
        let mut queue = VecDeque::new();

        // 鼠的必胜状态
        for j in 1..self.n {
            self.results[0][j][MOUSE_TURN] = Status::MOUSE;
            self.results[0][j][CAT_TURN] = Status::MOUSE;

            queue.push_back((0, j, MOUSE_TURN));
            queue.push_back((0, j, CAT_TURN));
        }

        // 猫的必胜状态
        for i in 1..self.n {
            self.results[i][i][MOUSE_TURN] = Status::CAT;
            self.results[i][i][CAT_TURN] = Status::CAT;

            queue.push_back((i, i, MOUSE_TURN));
            queue.push_back((i, i, CAT_TURN));
        }

        while let Some((mouse, cat, turn)) = queue.pop_front() {
            let result = self.results[mouse][cat][turn];

            for (last_mouse, last_cat, last_turn) in self.get_previous_states(mouse, cat, turn) {
                if self.results[last_mouse][last_cat][last_turn] == Status::DRAW {
                    if (result == Status::MOUSE && last_turn == MOUSE_TURN)
                        || (result == Status::CAT && last_turn == CAT_TURN)
                    {
                        self.results[last_mouse][last_cat][last_turn] = result;
                        queue.push_back((last_mouse, last_cat, last_turn));
                    } else {
                        self.degrees[last_mouse][last_cat][last_turn] -= 1;

                        if self.degrees[last_mouse][last_cat][last_turn] == 0 {
                            self.results[last_mouse][last_cat][last_turn] =
                                if last_turn == MOUSE_TURN {
                                    Status::CAT
                                } else {
                                    Status::MOUSE
                                };
                            queue.push_back((last_mouse, last_cat, last_turn));
                        }
                    }
                }
            }
        }

        match self.results[1][2][MOUSE_TURN] {
            Status::DRAW => 0,
            Status::MOUSE => 1,
            Status::CAT => 2,
        }
    }

    // Return last (mouse, cat, turn)
    fn get_previous_states(
        &self,
        mouse: usize,
        cat: usize,
        turn: usize,
    ) -> Vec<(usize, usize, usize)> {
        let previous_turn = if turn == MOUSE_TURN {
            CAT_TURN
        } else {
            MOUSE_TURN
        };

        if previous_turn == MOUSE_TURN {
            self.graph[mouse]
                .iter()
                .map(|x| (*x, cat, previous_turn))
                .collect()
        } else {
            self.graph[cat]
                .iter()
                .filter_map(|x| {
                    if x == &0 {
                        None
                    } else {
                        Some((mouse, *x, previous_turn))
                    }
                })
                .collect()
        }
    }
}

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let mut gamer = Gamer::new(graph);

        gamer.search()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_913() {
        assert_eq!(
            1,
            Solution::cat_mouse_game(vec![vec![1, 3], vec![0], vec![3], vec![0, 2]])
        );
    }
}
