/**
 * [3274] Check if Two Chessboard Squares Have the Same Color
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let mut c1 = coordinate1.chars();
        let mut c2 = coordinate2.chars();
        Self::get_color(c1.next().unwrap(), c1.next().unwrap())
            == Self::get_color(c2.next().unwrap(), c2.next().unwrap())
    }

    fn get_color(x: char, y: char) -> bool {
        let y = y.to_digit(10).unwrap();
        match x {
            'a' | 'c' | 'e' | 'g' => y % 2 == 1,
            'b' | 'd' | 'f' | 'h' => y % 2 == 0,
            _ => unreachable!(),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3274() {
        assert!(Solution::check_two_chessboards(
            "a1".to_owned(),
            "c3".to_owned()
        ));
        assert!(!Solution::check_two_chessboards(
            "a1".to_owned(),
            "h3".to_owned()
        ));
    }
}
