/**
 * [1184] Distance Between Bus Stops
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (start, destination) = (start as usize, destination as usize);
        let n = distance.len();
        
        let mut positive = start;
        let mut positive_result = 0;

        while positive != destination {
            positive_result += distance[positive];
            positive = (positive + 1) % n;
        }
        
        let mut negative = start;
        let mut negative_result = 0;

        while negative != destination {
            negative = (negative + n - 1) % n;
            negative_result += distance[negative]
        }
        
        positive_result.min(negative_result)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1184() {
        assert_eq!(1, Solution::distance_between_bus_stops(vec![1,2,3,4], 0, 1));
        assert_eq!(3, Solution::distance_between_bus_stops(vec![1,2,3,4],0, 2));
        assert_eq!(4, Solution::distance_between_bus_stops(vec![1,2,3,4],0,3));
    }
}
