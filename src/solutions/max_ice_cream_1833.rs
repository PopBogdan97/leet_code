//! It is a sweltering summer day, and a boy wants to buy some ice cream bars.
//!
//! At the store, there are n ice cream bars. You are given an array costs of length n, where
//! costs[i] is the price of the ith ice cream bar in coins. The boy initially has coins coins to
//! spend, and he wants to buy as many ice cream bars as possible.
//!
//! Return the maximum number of ice cream bars the boy can buy with coins coins.
//!
//! Note: The boy can buy the ice cream bars in any order.
//!
//!
//!
//! Example 1:
//!
//! Input: costs = [1,3,2,4,1], coins = 7
//! Output: 4
//! Explanation: The boy can buy ice cream bars at indices 0,1,2,4 for a total price of 1 + 3 + 2 +
//! 1 = 7.
//!
//! Example 2:
//!
//! Input: costs = [10,6,8,7,7,8], coins = 5
//! Output: 0
//! Explanation: The boy cannot afford any of the ice cream bars.
//!
//! Example 3:
//!
//! Input: costs = [1,6,3,1,2,5], coins = 20
//! Output: 6
//! Explanation: The boy can buy all the ice cream bars for a total price of 1 + 6 + 3 + 1 + 2 + 5
//! = 18.
//!
//!
//!
//! Constraints:
//!
//!     costs.length == n
//!         1 <= n <= 105
//!             1 <= costs[i] <= 105
//!                 1 <= coins <= 108
//!

mod max_ice_cream_1833 {
    /// In order to buy as meny ice creams as posible we need to buy first the ice-creams from the
    /// places where it costs less. Steps:
    /// 1. Order the costs of the ice-creams
    /// 2. Take the ice-creams until you reach the coins limit.

    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        let mut coins = coins;
        let mut result = 0;

        // Order the costs of the ice-creams.
        costs.sort();

        // Take all the posible ice-creams until the limit is reached.
        for c  in costs {
            if c <= coins {
                coins -= c;
                result += 1;
            } else {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::max_ice_cream_1833::max_ice_cream;

    #[test]
    fn test_example_1() {
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;

        assert_eq!(4, max_ice_cream(costs, coins));
    }

    #[test]
    fn test_example_2() {
        let costs = vec![10, 6, 8, 7, 7, 8];
        let coins = 5;

        assert_eq!(0, max_ice_cream(costs, coins));
    }

    #[test]
    fn test_example_3() {
        let costs = vec![1, 6, 3, 1, 2, 5];
        let coins = 20;

        assert_eq!(6, max_ice_cream(costs, coins));
    }
}
