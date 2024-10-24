/**
 * [2286] Booking Concert Tickets in Groups
 */
pub struct Solution {}

// submission codes start here

struct BookMyShow {
    row_count: i32,
    column_count: i32,
    min_tree: Vec<i32>,
    sum_tree: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let segement_depth = n as usize * 4;
        Self {
            row_count: n,
            column_count: m,
            min_tree: vec![0; segement_depth],
            sum_tree: vec![0; segement_depth],
        }
    }

    fn modify(&mut self, i: usize, l: i32, r: i32, index: i32, val: i32) {
        if l == r {
            self.min_tree[i] = val;
            self.sum_tree[i] = val as i64;
            return;
        }

        let middle = (l + r) / 2;
        if index <= middle {
            self.modify(i * 2, l, middle, index, val);
        } else {
            self.modify(i * 2 + 1, middle + 1, r, index, val)
        }

        self.min_tree[i] = self.min_tree[i * 2].min(self.min_tree[i * 2 + 1]);
        self.sum_tree[i] = self.sum_tree[i * 2] + self.sum_tree[i * 2 + 1];
    }

    fn query_min_row(&self, i: usize, l: i32, r: i32, val: i32) -> i32 {
        if l == r {
            return if self.min_tree[i] > val {
                self.row_count
            } else {
                l
            };
        }

        let middle = (l + r) / 2;
        if self.min_tree[i * 2] <= val {
            self.query_min_row(i * 2, l, middle, val)
        } else {
            self.query_min_row(i * 2 + 1, middle + 1, r, val)
        }
    }

    fn query_sum(&self, i: usize, l: i32, r: i32, target_left: i32, target_right: i32) -> i64 {
        if target_left <= l && r <= target_right {
            return self.sum_tree[i];
        }

        let middle = (l + r) / 2;
        let mut result = 0;

        if middle >= target_left {
            result += self.query_sum(i * 2, l, middle, target_left, target_right)
        }

        if middle + 1 <= target_right {
            result += self.query_sum(i * 2 + 1, middle + 1, r, target_left, target_right);
        }

        result
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let target_row = self.query_min_row(1, 0, self.row_count - 1, self.column_count - k);
        if target_row > max_row {
            return vec![];
        }

        let used = self.query_sum(1, 0, self.row_count - 1, target_row, target_row) as i32;
        self.modify(1, 0, self.row_count - 1, target_row, used + k);
        vec![target_row, used]
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let used = self.query_sum(1, 0, self.row_count - 1, 0, max_row);
        if ((max_row + 1) as i64 * self.column_count as i64) - used < k as i64 {
            return false;
        }

        let mut target_row = self.query_min_row(1, 0, self.row_count - 1, self.column_count - 1);
        let mut k = k;
        loop {
            let used = self.query_sum(1, 0, self.row_count - 1, target_row, target_row) as i32;
            if self.column_count - used >= k {
                self.modify(1, 0, self.row_count - 1, target_row, used + k);
                break;
            }

            k -= self.column_count - used;
            self.modify(1, 0, self.row_count - 1, target_row, self.column_count);
            target_row += 1;
        }

        true
    }
}

/**
 * Your BookMyShow object will be instantiated and called as such:
 * let obj = BookMyShow::new(n, m);
 * let ret_1: Vec<i32> = obj.gather(k, maxRow);
 * let ret_2: bool = obj.scatter(k, maxRow);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2286_1() {
        let mut show = BookMyShow::new(2, 5);

        assert_eq!(vec![0, 0], show.gather(4, 0));
        assert_eq!(Vec::<i32>::new(), show.gather(2, 0));
        assert!(show.scatter(5, 1));
        assert!(!show.scatter(5, 1));
    }

    #[test]
    fn test_2286_2() {
        let mut show = BookMyShow::new(5, 9);

        assert_eq!(Vec::<i32>::new(), show.gather(10, 1));
        assert!(show.scatter(3, 3));
        assert_eq!(vec![1, 0], show.gather(9, 1));
        assert_eq!(Vec::<i32>::new(), show.gather(10, 2));
        assert_eq!(vec![0, 3], show.gather(2, 0));
    }
}
