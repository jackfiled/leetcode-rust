/**
 * [855] Exam Room
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Debug, Eq, PartialEq)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn distance(&self) -> i32 {
        (self.end - self.start) / 2
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        // Interval将会被放入BinaryHeap中进行排序
        // 所以先返回middle的大
        // 再返回start小的
        match self.distance().cmp(&other.distance()) {
            Ordering::Equal => other.start.cmp(&self.start),
            r => r,
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

struct ExamRoom {
    n: i32,
    seats: BTreeSet<i32>,
    intervals: BinaryHeap<Interval>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            n,
            seats: BTreeSet::new(),
            intervals: BinaryHeap::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.seats.is_empty() {
            self.seats.insert(0);
            return 0;
        }

        // 坐在最左边到左边同学的距离
        let left = *self.seats.iter().next().unwrap();
        let right = self.n - 1 - *self.seats.iter().rev().next().unwrap();

        // 当当前坐着的同学数量大于等于2
        while self.seats.len() >= 2 {
            if let Some(interval) = self.intervals.peek() {
                // 判断是否需要延迟删除该区间
                if !self.seats.contains(&interval.start)
                    || !self.seats.contains(&interval.end)
                    || *self.seats.range((interval.start + 1)..).next().unwrap() != interval.end
                {
                    self.intervals.pop();
                    continue;
                }

                // 判断坐在区间的中间是否比坐在两个端点更好
                let distance = interval.distance();
                // 小于等于 left 是因为当距离一致时坐在较小的地方
                if distance < right || distance <= left {
                    break;
                }

                // 选择坐在中间
                let interval = self.intervals.pop().unwrap();
                let middle = interval.start + distance;
                self.intervals.push(Interval::new(interval.start, middle));
                self.intervals.push(Interval::new(middle, interval.end));
                self.seats.insert(middle);
                return middle;
            }
        }

        // 只有一个同学
        if right > left {
            let last = *self.seats.last().unwrap();
            self.intervals.push(Interval::new(last, self.n - 1));
            self.seats.insert(self.n - 1);
            self.n - 1
        } else {
            let first = *self.seats.first().unwrap();
            self.intervals.push(Interval::new(0, first));
            self.seats.insert(0);
            0
        }
    }

    fn leave(&mut self, p: i32) {
        // 如果学生不是最左或者是最右的区间
        // 那么删除会产生新的区间
        // 判断区间是否有效是Self::seat()的工作
        if p != *self.seats.first().unwrap() && p != *self.seats.last().unwrap() {
            let pre = *self.seats.range(..p).last().unwrap();
            let next = *self.seats.range((p + 1)..).next().unwrap();

            self.intervals.push(Interval::new(pre, next));
        }

        self.seats.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_855() {
        let mut room = ExamRoom::new(10);
        assert_eq!(0, room.seat());
        assert_eq!(9, room.seat());
        assert_eq!(4, room.seat());
        assert_eq!(2, room.seat());
        room.leave(4);
        assert_eq!(5, room.seat());
    }
}
