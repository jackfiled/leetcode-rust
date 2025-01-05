/**
 * [2241] Design an ATM Machine
 */
pub struct Solution {}

// submission codes start here

struct ATM {
    money: Vec<i32>,
    denomination: [i32; 5],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        Self {
            money: vec![0; 5],
            denomination: [20, 50, 100, 200, 500],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, v) in banknotes_count.into_iter().enumerate() {
            self.money[i] += v;
        }
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut result = vec![0; 5];

        for (i, &v) in self.denomination.iter().enumerate().rev() {
            result[i] = (amount / v).min(self.money[i]);

            amount -= result[i] * v;
        }

        if amount != 0 {
            vec![-1]
        } else {
            for (i, &v) in result.iter().enumerate() {
                self.money[i] -= v;
            }
            result
        }
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2241() {
        let mut atm = ATM::new();

        atm.deposit(vec![0, 10, 0, 3, 0]);
        assert_eq!(vec![0, 2, 0, 2, 0], atm.withdraw(500));

        let mut atm = ATM::new();

        atm.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(vec![0, 0, 1, 0, 1], atm.withdraw(600));
        atm.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(vec![-1], atm.withdraw(600));
        assert_eq!(vec![0, 1, 0, 0, 1], atm.withdraw(550));
    }
}
