/**
 * [838] Push Dominoes
 */
pub struct Solution {}

// submission codes start here
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Left,
    Right,
    Untouched,
}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes: Vec<State> = dominoes
            .chars()
            .map(|c| match c {
                'L' => State::Left,
                'R' => State::Right,
                '.' => State::Untouched,
                _ => unimplemented!(),
            })
            .collect();
        let n = dominoes.len();

        // 为双指针插入头尾伪节点
        dominoes.insert(0, State::Untouched);
        dominoes.push(State::Untouched);

        let (mut left, mut right) = (0, 1);

        // 不能判断右指针
        // 左指针才是遍历完成的信息
        while left <= n {
            while right <= n && dominoes[right] == State::Untouched {
                right += 1;
            }

            // 找到没有推动的区间(left, right)
            if right - left > 1 {
                let (mut inner_left, mut inner_right) = (left + 1, right - 1);

                match (dominoes[left], dominoes[right]) {
                    (State::Untouched, State::Left) | (State::Left, State::Left) => {
                        while inner_left <= inner_right {
                            dominoes[inner_left] = State::Left;
                            inner_left += 1;
                        }
                    }
                    (State::Right, State::Untouched) | (State::Right, State::Right) => {
                        while inner_left <= inner_right {
                            dominoes[inner_left] = State::Right;
                            inner_left += 1;
                        }
                    }
                    (State::Right, State::Left) => {
                        while inner_left < inner_right {
                            // 注意需要同时修改
                            let l = Self::between_left_right(&mut dominoes, inner_left);
                            let r = Self::between_left_right(&mut dominoes, inner_right);
                            dominoes[inner_left] = l;
                            dominoes[inner_right] = r;
                            inner_left += 1;
                            inner_right -= 1;
                        }
                    }
                    _ => {}
                }
            }

            left = right;
            right += 1;
        }

        dominoes[1..=n]
            .iter()
            .map(|x| match x {
                State::Left => 'L',
                State::Right => 'R',
                State::Untouched => '.',
            })
            .collect()
    }

    fn between_left_right(dominoes: &mut Vec<State>, pos: usize) -> State {
        match (dominoes[pos - 1], dominoes[pos + 1]) {
            (State::Right, State::Untouched) => State::Right,
            (State::Untouched, State::Left) => State::Left,
            // 其他情况都不会影响中间的
            _ => dominoes[pos],
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_838() {
        assert_eq!("LL.RR", Solution::push_dominoes(".L.R.".to_owned()));
        assert_eq!("RR.L", Solution::push_dominoes("RR.L".to_owned()));
        assert_eq!(
            "LL.RR.LLRRLL..",
            Solution::push_dominoes(".L.R...LR..L..".to_owned())
        );
    }
}
