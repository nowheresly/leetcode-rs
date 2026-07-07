pub struct Solution {}

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let mut res = 0;
        let mut buffer = 0;
        let mut main_tank = main_tank;
        let mut additional_tank = additional_tank;
        while main_tank > 0 {
            main_tank -= 1;
            buffer += 1;
            res += 10;
            if additional_tank > 0 && buffer == 5 {
                buffer = 0;
                main_tank += 1;
                additional_tank -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(60, Solution::distance_traveled(5,  10));

    }
}
