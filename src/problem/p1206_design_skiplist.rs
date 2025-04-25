/**
 * [1206] Design Skiplist
 */
pub struct Solution {}

// submission codes start here
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

struct SkiplistNode {
    right: Option<Rc<RefCell<SkiplistNode>>>,
    down: Option<Rc<RefCell<SkiplistNode>>>,
    value: i32,
}

type NodeCell = Rc<RefCell<SkiplistNode>>;

struct Skiplist {
    head: Rc<RefCell<SkiplistNode>>,
    random_engine: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn create_node(value: i32) -> NodeCell {
        Rc::new(RefCell::new(SkiplistNode {
            right: None,
            down: None,
            value,
        }))
    }

    fn new() -> Self {
        Self {
            head: Self::create_node(-1),
            random_engine: rand::thread_rng(),
        }
    }

    fn search(&self, target: i32) -> bool {
        let mut node = Some(self.head.clone());
        let mut queue = vec![];

        while let Some(real_node) = node {
            node = if real_node
                .borrow()
                .right
                .clone()
                .filter(|x| x.borrow().value <= target)
                .is_some()
            {
                real_node.borrow().right.clone()
            } else {
                queue.push(real_node.clone());
                real_node.borrow().down.clone()
            };
        }

        queue
            .last()
            .filter(|x| x.borrow().value == target)
            .is_some()
    }

    fn add(&mut self, num: i32) {
        let mut path = vec![];
        let mut node = Some(self.head.clone());

        // 找链表中所有小于num的节点
        while let Some(real_node) = node {
            node = if real_node
                .borrow()
                .right
                .as_ref()
                .filter(|x| x.borrow().value <= num)
                .is_some()
            {
                real_node.borrow().right.clone()
            } else {
                path.push(real_node.clone());
                real_node.borrow().down.as_ref().map(|x| x.clone())
            };
        }

        let mut insert = true;
        let mut down_node = None;

        while insert && !path.is_empty() {
            if let Some(insert_node) = path.pop() {
                // 添加新的节点
                let right_node = insert_node.borrow_mut().right.take();
                insert_node.borrow_mut().right = Some(Rc::new(RefCell::new(SkiplistNode {
                    right: right_node,
                    down: down_node.take(),
                    value: num,
                })));

                down_node = insert_node.borrow().right.as_ref().map(|x| x.clone());
                // 50%的概率决定是否添加下一级索引
                insert = self.random_engine.gen_bool(0.5);
            } else {
                break;
            }
        }

        // 新增头结点
        // 这里新增一层
        if insert {
            let node = Skiplist::create_node(-1);
            node.borrow_mut().down = Some(self.head.clone());
            self.head = node;
        }
    }

    fn erase(&self, num: i32) -> bool {
        let mut deleted = false;
        let mut node = Some(self.head.clone());

        while let Some(mut real_node) = node {
            node = if real_node
                .borrow()
                .right
                .as_ref()
                .filter(|x| x.borrow().value < num)
                .is_some()
            {
                real_node.borrow().right.clone()
            } else {
                real_node.borrow().down.clone()
            };

            if real_node
                .borrow()
                .right
                .as_ref()
                .filter(|x| x.borrow().value == num)
                .is_some()
            {
                deleted = true;
                let right = real_node
                    .borrow_mut()
                    .right
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .right
                    .as_ref()
                    .map(|x| x.clone());
                real_node.borrow_mut().right = right;
            }
        }

        deleted
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1206() {
        let mut list = Skiplist::new();

        list.add(1);
        list.add(2);
        list.add(3);
        assert!(!list.search(0));
        list.add(4);
        assert!(list.search(1));
        assert!(!list.erase(0));
        assert!(list.erase(1));
        assert!(!list.search(1));
    }
}
