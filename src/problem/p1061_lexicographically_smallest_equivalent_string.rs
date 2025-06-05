/**
 * [1061] Lexicographically Smallest Equivalent String
 */
pub struct Solution {}

// submission codes start here

/// Use union find set for this question
struct EquivalentSet {
    parents: Vec<usize>,
    min_values: Vec<usize>,
}

impl std::fmt::Debug for EquivalentSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parents: \n")?;

        for (i, &v) in self.parents.iter().enumerate() {
            write!(
                f,
                "{} -> {}\n",
                Self::convert_to_char(i),
                Self::convert_to_char(v)
            )?;
        }

        writeln!(f, "min_values:")?;

        for (i, &v) in self.min_values.iter().enumerate() {
            writeln!(
                f,
                "{} -> {}",
                Self::convert_to_char(i),
                Self::convert_to_char(v)
            )?;
        }

        Ok(())
    }
}

impl EquivalentSet {
    fn new() -> Self {
        Self {
            // Only lower characters
            parents: (0..26).collect(),
            min_values: (0..26).collect(),
        }
    }

    fn convert_to_char(c: usize) -> char {
        (c as u8 + b'a') as char
    }

    fn find(&mut self, a: usize) -> (usize, usize) {
        if self.parents[a] == a {
            (a, self.min_values[a])
        } else {
            let (p, min_value) = self.find(self.parents[a]);
            self.parents[a] = p;
            self.min_values[a] = min_value;
            (p, min_value)
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let ((a_p, a_min_value), (b_p, b_min_value)) = (self.find(a), self.find(b));
        let min_value = a_min_value.min(b_min_value).min(a).min(b);

        self.parents[a_p] = b_p;
        self.min_values[a_p] = min_value;
        self.min_values[b_p] = min_value;
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut set = EquivalentSet::new();

        for (a, b) in s1.bytes().zip(s2.bytes()) {
            set.merge((a - b'a') as usize, (b - b'a') as usize);
        }

        String::from_utf8(
            base_str
                .bytes()
                .map(|c| {
                    let (_, min_value) = set.find((c - b'a') as usize);

                    min_value as u8 + b'a'
                })
                .collect(),
        )
        .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1061() {
        assert_eq!(
            "makkek",
            Solution::smallest_equivalent_string(
                "parker".to_string(),
                "morris".to_string(),
                "parser".to_string()
            )
        );
        assert_eq!(
            "hdld",
            Solution::smallest_equivalent_string(
                "hello".to_string(),
                "world".to_string(),
                "hold".to_string()
            )
        );
        assert_eq!(
            "aauaaaaada".to_string(),
            Solution::smallest_equivalent_string(
                "leetcode".to_string(),
                "programs".to_string(),
                "sourcecode".to_string()
            )
        );
    }
}
