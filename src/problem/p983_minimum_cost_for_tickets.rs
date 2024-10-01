/**
 * [983] Minimum Cost For Tickets
 */
pub struct Solution {}


// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut memory = vec![None; 366];
        let days: HashSet<usize> = days.iter().map(|x| *x as usize).collect();
        
        Self::dp(1, &mut memory, &days, &costs)
    }
    
    fn dp(day: usize, memory: &mut Vec<Option<i32>>, days: &HashSet<usize>, costs: &Vec<i32>) -> i32 {
        if day > 365 {
            return 0;
        }
        
        if let Some(result) = memory[day] {
            return result;
        }
        
        let result = if days.contains(&day) {
            (Self::dp(day + 1, memory, days, costs) + costs[0])
                .min(Self::dp(day + 7, memory, days, costs) + costs[1])
                .min(Self::dp(day + 30, memory, days, costs) + costs[2])
        } else { 
            Self::dp(day + 1, memory, days, costs)
        };
        
        memory[day] = Some(result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_983() {
        assert_eq!(11, Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]));
        assert_eq!(17, Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]));
    }
}
