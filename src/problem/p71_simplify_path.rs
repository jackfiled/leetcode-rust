/**
 * [71] Simplify Path
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let names = path.split('/');
        let mut path = vec![];

        for name in names {
            if name.len() == 0 {
                continue;
            }

            if name == "." {
                continue;
            } else if name == ".." {
                path.pop();
            } else {
                path.push(name)
            }
        }

        let mut result = String::new();

        for name in path {
            result.push_str("/");
            result.push_str(name);
        }

        if result.len() == 0 {
            "/".to_owned()
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_71() {
        assert_eq!(
            "/home".to_owned(),
            Solution::simplify_path("/home//".to_owned())
        );
    }
}
