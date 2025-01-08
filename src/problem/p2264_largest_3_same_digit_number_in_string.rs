/**
 * [2264] Largest 3-Same-Digit Number in String
 */
pub struct Solution {}

// submission codes start here

const NUMBERS: [&str; 10] = [
    "999", "888", "777", "666", "555", "444", "333", "222", "111", "000",
];

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        for n in NUMBERS {
            if num.contains(n) {
                return n.to_owned();
            }
        }

        "".to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2264() {
        assert_eq!(
            "777".to_owned(),
            Solution::largest_good_integer("6777133339".to_owned())
        );
        assert_eq!(
            "000".to_owned(),
            Solution::largest_good_integer("2300019".to_owned())
        );
        assert_eq!(
            "".to_owned(),
            Solution::largest_good_integer("42352338".to_owned())
        );
    }
}
