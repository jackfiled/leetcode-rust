/**
 * [670] Maximum Swap
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num_array = Vec::new();
        let mut num = num;

        while num != 0 {
            num_array.push(num % 10);
            num = num / 10;
        }
        num_array.reverse();

        for i in 0..num_array.len() {
            let mut index = i;
            for j in (i + 1)..num_array.len() {
                if num_array[j] >= num_array[index] {
                    index = j;
                }
            }

            if index != i && num_array[index] != num_array[i] {
                let temp = num_array[i];
                num_array[i] = num_array[index];
                num_array[index] = temp;
                break;
            }
        }

        let mut result = 0;

        for i in num_array {
            result += i;
            result *= 10;
        }
        result /= 10;

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_670() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(9973), 9973);
        assert_eq!(Solution::maximum_swap(98368), 98863);
        assert_eq!(Solution::maximum_swap(1993), 9913);
    }
}
