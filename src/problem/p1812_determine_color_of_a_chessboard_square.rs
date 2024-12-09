/**
 * [1812] Determine Color of a Chessboard Square
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let mut coordinates = coordinates.chars();
        let (x, y) = (coordinates.next().unwrap(), coordinates.next().unwrap());

        match x {
            'a' | 'c' | 'e' | 'g' => match y {
                '1' | '3' | '5' | '7' => false,
                '2' | '4' | '6' | '8' => true,
                _ => unimplemented!(),
            },
            'b' | 'd' | 'f' | 'h' => match y {
                '1' | '3' | '5' | '7' => true,
                '2' | '4' | '6' | '8' => false,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1812() {
        assert!(!Solution::square_is_white("a1".to_owned()));
        assert!(Solution::square_is_white("h3".to_owned()));
        assert!(!Solution::square_is_white("c7".to_owned()));
    }
}
