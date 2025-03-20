use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn assert_array_unorder_equal<T>(a: &Vec<T>, b: &Vec<T>)
where
    T: Eq,
    T: Hash,
{
    assert_eq!(a.len(), b.len());

    let set: HashSet<&T> = HashSet::from_iter(a.into_iter());

    for i in b.iter() {
        assert!(set.contains(i));
    }
}

#[macro_export]
macro_rules! assert_array_unorder_equal {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_value, right_value) => {
                $crate::util::vec_equal::assert_array_unorder_equal(left_value, right_value);
            }
        }
    };
}

pub fn assert_inner_array_unorder_equal<T>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>)
where
    T: Eq,
    T: Hash,
{
    assert_eq!(a.len(), b.len());

    for (i, j) in a.iter().zip(b.iter()) {
        assert_array_unorder_equal(i, j);
    }
}

#[macro_export]
macro_rules! assert_inner_array_unorder_equal {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_value, right_value) => {
                $crate::util::vec_equal::assert_inner_array_unorder_equal(left_value, right_value);
            }
        }
    };
}
