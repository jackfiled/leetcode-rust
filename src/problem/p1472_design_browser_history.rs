/**
 * [1472] Design Browser History
 */
pub struct Solution {}

// submission codes start here

struct BrowserHistory {
    history: Vec<String>,
    pos: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            pos: 0,
        }
    }

    fn visit(&mut self, url: String) {
        // 删除前向的所有网站
        for _ in self.pos + 1..self.history.len() {
            self.history.pop();
        }

        self.history.push(url);
        self.pos += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        if steps > self.pos {
            self.pos = 0;
        } else {
            self.pos -= steps;
        }

        self.history[self.pos].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        if steps + self.pos >= self.history.len() {
            self.pos = self.history.len() - 1;
        } else {
            self.pos += steps;
        }

        self.history[self.pos].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1472() {
        let mut history = BrowserHistory::new("leetcode.com".to_owned());

        history.visit("google.com".to_owned());
        history.visit("facebook.com".to_owned());
        history.visit("youtube.com".to_owned());
        assert_eq!("facebook.com", history.back(1));
        assert_eq!("google.com", history.back(1));
        assert_eq!("facebook.com", history.forward(1));
        history.visit("linkedin.com".to_owned());
        assert_eq!("linkedin.com", history.forward(2));
        assert_eq!("google.com", history.back(2));
        assert_eq!("leetcode.com", history.back(7));
    }
}
