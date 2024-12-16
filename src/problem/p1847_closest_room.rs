/**
 * [1847] Closest Room
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum EventTypeKind {
    Room,
    Query,
}

struct Event {
    event_type: EventTypeKind,
    size: i32,
    id: i32,
    origin: usize,
}

impl Event {
    fn new_room(i: usize, room: &Vec<i32>) -> Self {
        Self {
            event_type: EventTypeKind::Room,
            size: room[1],
            id: room[0],
            origin: i,
        }
    }

    fn new_query(i: usize, query: &Vec<i32>) -> Self {
        Self {
            event_type: EventTypeKind::Query,
            size: query[1],
            id: query[0],
            origin: i,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.event_type == other.event_type
    }
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .size
            .cmp(&self.size)
            .then(self.event_type.cmp(&other.event_type))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut events: Vec<Event> = rooms
            .iter()
            .enumerate()
            .map(|(i, room)| Event::new_room(i, room))
            .chain(
                queries
                    .iter()
                    .enumerate()
                    .map(|(i, query)| Event::new_query(i, query)),
            )
            .collect();

        events.sort();
        let mut valid_rooms = BTreeSet::new();
        let mut result = vec![-1; queries.len()];

        for event in events {
            if event.event_type == EventTypeKind::Room {
                valid_rooms.insert(event.id);
            } else {
                let mut distance = i32::MAX;

                if let Some(&ceil) = valid_rooms.range(event.id..).next() {
                    if ceil - event.id < distance {
                        distance = ceil - event.id;
                        result[event.origin] = ceil;
                    }
                }

                if let Some(&floor) = valid_rooms.range(..event.id).next_back() {
                    if event.id - floor <= distance {
                        result[event.origin] = floor;
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1847() {
        assert_eq!(
            vec![3, -1, 3],
            Solution::closest_room(
                vec![vec![2, 2], vec![1, 2], vec![3, 2]],
                vec![vec![3, 1], vec![3, 3], vec![5, 2]]
            )
        );
    }
}
