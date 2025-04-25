/**
 * [1656] Design an Ordered Stream
 */
pub struct Solution {}

// submission codes start here

struct OrderedStream {
    array: Vec<Option<String>>,
    pointer: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        let n = n as usize;

        Self {
            array: vec![None; n],
            pointer: 1,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id = id_key as usize - 1;

        self.array[id] = Some(value);

        let mut result = vec![];
        let mut pointer = self.pointer;

        while pointer <= self.array.len() {
            if let Some(item) = &self.array[pointer - 1] {
                result.push(item.clone());
                pointer += 1;
            } else {
                break;
            }
        }
        self.pointer = pointer;

        result
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1656() {
        let mut stream = OrderedStream::new(5);

        assert_eq!(Vec::<&str>::new(), stream.insert(3, "ccccc".to_owned()));
        assert_eq!(vec!["aaaaa"], stream.insert(1, "aaaaa".to_owned()));
        assert_eq!(vec!["bbbbb", "ccccc"], stream.insert(2, "bbbbb".to_owned()));
        assert_eq!(Vec::<&str>::new(), stream.insert(5, "eeeee".to_owned()));
        assert_eq!(vec!["ddddd", "eeeee"], stream.insert(4, "ddddd".to_owned()));
    }
}
