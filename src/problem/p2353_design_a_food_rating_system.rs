/**
 * [2353] Design a Food Rating System
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Food {
    name: String,
    cuisine: String,
    rating: i32,
}

impl Food {
    fn new(name: String, cuisine: String, rating: i32) -> Self {
        Self {
            name,
            cuisine,
            rating,
        }
    }

    fn update_rating(&self, new_rating: i32) -> Self {
        Self {
            name: self.name.clone(),
            cuisine: self.cuisine.clone(),
            rating: new_rating,
        }
    }
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rating.cmp(&other.rating) {
            Ordering::Equal => other.name.cmp(&self.name),
            a => a,
        }
    }
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct FoodRatings {
    food_map: HashMap<String, Food>,
    cuisine_heap: HashMap<String, BinaryHeap<Food>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_map = HashMap::new();
        let mut cuisine_heap = HashMap::new();

        for ((name, cuisine), &rating) in foods.iter().zip(cuisines.iter()).zip(ratings.iter()) {
            food_map.insert(
                name.clone(),
                Food::new(name.clone(), cuisine.clone(), rating),
            );

            let heap = cuisine_heap
                .entry(cuisine.clone())
                .or_insert(BinaryHeap::new());
            heap.push(Food::new(name.clone(), cuisine.clone(), rating.clone()));
        }

        Self {
            food_map,
            cuisine_heap,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let food_entry = self.food_map.get_mut(&food).unwrap();
        food_entry.rating = new_rating;

        let heap = self.cuisine_heap.get_mut(&food_entry.cuisine).unwrap();
        heap.push(food_entry.update_rating(new_rating));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.cuisine_heap.get_mut(&cuisine).unwrap();

        while let Some(head) = heap.peek() {
            let food_entry = self.food_map.get(&head.name).unwrap();

            if head.rating == food_entry.rating {
                return food_entry.name.clone();
            }

            heap.pop();
        }

        "".to_owned()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2353() {
        let mut ratings = FoodRatings::new(
            vec_string!("kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"),
            vec_string!("korean", "japanese", "japanese", "greek", "japanese", "korean"),
            vec![9, 12, 8, 15, 14, 7],
        );

        assert_eq!("kimchi", ratings.highest_rated("korean".to_owned()));
        assert_eq!("ramen", ratings.highest_rated("japanese".to_owned()));

        ratings.change_rating("sushi".to_owned(), 16);
        assert_eq!("sushi", ratings.highest_rated("japanese".to_owned()));

        ratings.change_rating("ramen".to_owned(), 16);
        assert_eq!("ramen", ratings.highest_rated("japanese".to_owned()));
    }
}
