/**
 * [2332] The Latest Time to Catch a Bus
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let (mut buses, mut passengers) = (buses, passengers);
        buses.sort();
        passengers.sort();
        let (n, m) = (buses.len(), passengers.len());

        let mut last_passenger = 0;
        let mut now_passengers = 0;
        let mut last_passengers = 0;

        // 找到最后一位上车的乘客和当前车上人员的数量
        for &bus in buses.iter() {
            while last_passenger < m && passengers[last_passenger] <= bus {
                now_passengers += 1;
                last_passenger += 1;

                if now_passengers == capacity {
                    break;
                }
            }

            // 无论坐没坐满都要发车
            last_passengers = now_passengers;
            now_passengers = 0;
        }

        // 最后一个上车的哥们
        let mut down_passenger = if last_passenger == 0 {
            // 特判一下如果没有哥们可以上车的情况
            return buses[n - 1];
        } else {
            last_passenger - 1
        };

        let mut result = if last_passengers < capacity {
            // 最后一辆车上还有空位
            // 直接在最后一辆车来之前上车
            // 他妈的这里也要考虑重复到达的问题
            buses[n - 1]
        } else {
            // 如果最后一辆车也上满了
            // 那么就把最后一个上车的哥们挤下去
            let mut result = passengers[down_passenger] - 1;

            // 在这种情况下，考虑重复时需要考虑那个哥们已经被挤下去了
            if down_passenger == 0 {
                return result;
            }

            down_passenger -= 1;

            result
        };

        loop {
            if result != passengers[down_passenger] {
                break;
            }

            result -= 1;
            if down_passenger == 0 {
                break;
            }
            down_passenger -= 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2332() {
        assert_eq!(
            16,
            Solution::latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2)
        );
        assert_eq!(
            20,
            Solution::latest_time_catch_the_bus(
                vec![20, 30, 10],
                vec![19, 13, 26, 4, 25, 11, 21],
                2
            )
        );
        assert_eq!(
            1,
            Solution::latest_time_catch_the_bus(vec![3], vec![2, 3], 2)
        );
        assert_eq!(1, Solution::latest_time_catch_the_bus(vec![2], vec![2], 1));
        assert_eq!(1, Solution::latest_time_catch_the_bus(vec![2], vec![2], 2));
        assert_eq!(3, Solution::latest_time_catch_the_bus(vec![3], vec![4], 1));
    }
}
