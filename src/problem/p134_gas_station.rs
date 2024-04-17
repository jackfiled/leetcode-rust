/**
 * [134] Gas Station
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let length = gas.len();
        let mut value = Vec::with_capacity(length * 2);

        for i in 0..length {
            value.push(gas[i] - cost[i]);
        }

        if value.iter().sum::<i32>() < 0 {
            return -1;
        }

        for i in 0..length {
            value.push(value[i]);
        }

        let mut pos = 0;
        while pos < length {
            let mut g = 0;
            let mut flag = true;

            for i in 0..length {
                g += value[pos + i];

                if g < 0 {
                    pos = pos + i + 1;
                    flag = false;
                    break;
                }
            }

            if flag {
                return pos as i32;
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_134() {
        assert_eq!(
            3,
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
        assert_eq!(
            -1,
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
        );
    }
}
