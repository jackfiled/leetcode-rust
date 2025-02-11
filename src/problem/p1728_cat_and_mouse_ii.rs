/**
 * [1728] Cat and Mouse II
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

const MOUSE_TURN: usize = 0;
const CAT_TURN: usize = 1;
const MAX_ROUND: i32 = 1000;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Status {
    DRAW,
    MOUSE,
    CAT,
}

#[derive(Copy, Clone, Debug)]
struct Result {
    status: Status,
    count: i32,
}

impl Result {
    fn new() -> Self {
        Self {
            status: Status::DRAW,
            count: 0,
        }
    }

    fn new_with_status(status: Status) -> Self {
        Self { status, count: 0 }
    }

    fn next(&self) -> Self {
        Self {
            status: self.status,
            count: self.count + 1,
        }
    }
}

struct Gamer {
    row: usize,
    column: usize,
    food: (usize, usize),
    grid: Vec<Vec<char>>,
    mouse_jump: i32,
    cat_jump: i32,
    degrees: Vec<Vec<Vec<i32>>>,
    results: Vec<Vec<Vec<Result>>>,
}

impl Gamer {
    // (Self, start_mouse_pos, start_cat_pos)
    fn new(
        grid: Vec<String>,
        cat_jump: i32,
        mouse_jump: i32,
    ) -> (Self, (usize, usize), (usize, usize)) {
        let grid = grid
            .into_iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let (row, column) = (grid.len(), grid[0].len());
        let (mut start_mouse, mut start_cat, mut food) = ((0, 0), (0, 0), (0, 0));

        for i in 0..row {
            for j in 0..column {
                match grid[i][j] {
                    'M' => start_mouse = (i, j),
                    'C' => start_cat = (i, j),
                    'F' => food = (i, j),
                    _ => {}
                }
            }
        }

        let total = row * column;
        let mut degrees = vec![vec![vec![0; 2]; total]; total];
        let mut results = vec![vec![vec![Result::new(); 2]; total]; total];

        // 计算每个状态的度
        for mouse_i in 0..row {
            for mouse_j in 0..column {
                let mouse_state = mouse_i * column + mouse_j;
                if grid[mouse_i][mouse_j] == '#' {
                    continue;
                }

                for cat_i in 0..row {
                    for cat_j in 0..column {
                        let cat_state = cat_i * column + cat_j;
                        if grid[cat_i][cat_j] == '#' {
                            continue;
                        }

                        degrees[mouse_state][cat_state][MOUSE_TURN] += 1;
                        degrees[mouse_state][cat_state][CAT_TURN] += 1;

                        for dir in DIRECTIONS.iter() {
                            let (mouse_i, mouse_j) = (mouse_i as i32, mouse_j as i32);

                            // 让老鼠先跳
                            for jump in 1..=mouse_jump {
                                let (i, j) = (mouse_i + jump * dir.0, mouse_j + jump * dir.1);

                                if i >= 0 && i < row as i32 && j >= 0 && j < column as i32 {
                                    let (i, j) = (i as usize, j as usize);
                                    if grid[i][j] == '#' {
                                        break;
                                    }
                                    degrees[i * column + j][cat_state][MOUSE_TURN] += 1;
                                }
                            }

                            let (cat_i, cat_j) = (cat_i as i32, cat_j as i32);

                            // 让猫跳
                            for jump in 1..=cat_jump {
                                let (i, j) = (cat_i + jump * dir.0, cat_j + jump * dir.1);

                                if i >= 0 && i < row as i32 && j >= 0 && j < column as i32 {
                                    let (i, j) = (i as usize, j as usize);
                                    if grid[i][j] == '#' {
                                        break;
                                    }
                                    degrees[mouse_state][i * column + j][CAT_TURN] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        (
            Self {
                row,
                column,
                food,
                grid,
                degrees,
                results,
                mouse_jump,
                cat_jump,
            },
            start_mouse,
            start_cat,
        )
    }

    fn get_state(&self, (x, y): (usize, usize)) -> usize {
        self.column * x + y
    }

    fn search(&mut self, start_mouse: (usize, usize), start_cat: (usize, usize)) -> bool {
        let mut queue = VecDeque::new();

        // 猫和老鼠在同一位置
        // 猫必胜
        for i in 0..self.row {
            for j in 0..self.column {
                if self.grid[i][j] == '#' {
                    continue;
                }

                let state = self.get_state((i, j));
                self.results[state][state][MOUSE_TURN] = Result::new_with_status(Status::CAT);
                self.results[state][state][CAT_TURN] = Result::new_with_status(Status::CAT);

                queue.push_back((state, state, MOUSE_TURN));
                queue.push_back((state, state, CAT_TURN));
            }
        }

        // 猫和食物在同一位置
        // 猫必胜
        let food_state = self.get_state(self.food);
        for i in 0..self.row {
            for j in 0..self.column {
                if self.grid[i][j] == '#' || (i, j) == self.food {
                    continue;
                }

                let mouse_state = self.get_state((i, j));
                self.results[mouse_state][food_state][MOUSE_TURN] =
                    Result::new_with_status(Status::CAT);
                self.results[mouse_state][food_state][CAT_TURN] =
                    Result::new_with_status(Status::CAT);

                queue.push_back((mouse_state, food_state, MOUSE_TURN));
                queue.push_back((mouse_state, food_state, CAT_TURN));
            }
        }

        // 老鼠和食物在同一位置且猫不在
        // 老鼠必胜
        for i in 0..self.row {
            for j in 0..self.column {
                if self.grid[i][j] == '#' || (i, j) == self.food {
                    continue;
                }

                let cat_state = self.get_state((i, j));
                self.results[food_state][cat_state][MOUSE_TURN] =
                    Result::new_with_status(Status::MOUSE);
                self.results[food_state][cat_state][CAT_TURN] =
                    Result::new_with_status(Status::MOUSE);

                queue.push_back((food_state, cat_state, MOUSE_TURN));
                queue.push_back((food_state, cat_state, CAT_TURN));
            }
        }

        // 开始拓扑排序
        while let Some((mouse_state, cat_state, turn)) = queue.pop_front() {
            let result = self.results[mouse_state][cat_state][turn];

            for (previous_mouse_state, previous_cat_state, previous_turn) in
                self.get_previous_states(mouse_state, cat_state, turn)
            {
                if self.results[previous_mouse_state][previous_cat_state][previous_turn].status
                    == Status::DRAW
                {
                    if (result.status == Status::MOUSE && previous_turn == MOUSE_TURN)
                        || (result.status == Status::CAT && previous_turn == CAT_TURN)
                    {
                        self.results[previous_mouse_state][previous_cat_state][previous_turn] =
                            result.next();

                        queue.push_back((previous_mouse_state, previous_cat_state, previous_turn));
                    } else {
                        self.degrees[previous_mouse_state][previous_cat_state][previous_turn] -= 1;

                        if self.degrees[previous_mouse_state][previous_cat_state][previous_turn]
                            == 0
                        {
                            self.results[previous_mouse_state][previous_cat_state][previous_turn] =
                                Result {
                                    status: if previous_turn == MOUSE_TURN {
                                        Status::CAT
                                    } else {
                                        Status::MOUSE
                                    },
                                    count: result.count + 1,
                                };

                            queue.push_back((
                                previous_mouse_state,
                                previous_cat_state,
                                previous_turn,
                            ));
                        }
                    }
                }
            }
        }

        let result =
            &self.results[self.get_state(start_mouse)][self.get_state(start_cat)][MOUSE_TURN];
        result.status == Status::MOUSE && result.count <= MAX_ROUND
    }

    // (mouse, cat, turn)
    fn get_previous_states(
        &self,
        mouse_state: usize,
        cat_state: usize,
        turn: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut states = vec![];

        let (mouse_x, mouse_y) = (
            (mouse_state / self.column) as i32,
            (mouse_state % self.column) as i32,
        );
        let (cat_x, cat_y) = (
            (cat_state / self.column) as i32,
            (cat_state % self.column) as i32,
        );

        let previous_turn = if turn == MOUSE_TURN {
            CAT_TURN
        } else {
            MOUSE_TURN
        };
        let jump = if previous_turn == MOUSE_TURN {
            self.mouse_jump
        } else {
            self.cat_jump
        };
        let (start_x, start_y) = if previous_turn == MOUSE_TURN {
            (mouse_x, mouse_y)
        } else {
            (cat_x, cat_y)
        };

        states.push((mouse_state, cat_state, previous_turn));

        for dir in DIRECTIONS.iter() {
            for j in 1..=jump {
                let (x, y) = (start_x + j * dir.0, start_y + j * dir.1);
                if x >= 0 && x < self.row as i32 && y >= 0 && y < self.column as i32 {
                    let (x, y) = (x as usize, y as usize);

                    if self.grid[x][y] == '#' {
                        break;
                    }

                    let previous_mouse = if previous_turn == MOUSE_TURN {
                        (x, y)
                    } else {
                        (mouse_x as usize, mouse_y as usize)
                    };
                    let previous_cat = if previous_turn == CAT_TURN {
                        (x, y)
                    } else {
                        (cat_x as usize, cat_y as usize)
                    };

                    states.push((
                        self.get_state(previous_mouse),
                        self.get_state(previous_cat),
                        previous_turn,
                    ));
                }
            }
        }

        states
    }
}

impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let (mut gamer, start_mouse, start_cat) = Gamer::new(grid, cat_jump, mouse_jump);
        gamer.search(start_mouse, start_cat)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1728() {
        assert!(!Solution::can_mouse_win(
            vec_string!("C...#", "...#F", "....#", "M...."),
            2,
            5
        ));
        assert!(Solution::can_mouse_win(vec_string!("M.C...F"), 1, 4));
        assert!(Solution::can_mouse_win(
            vec_string!("####F", "#C...", "M...."),
            1,
            2
        ));
    }
}
