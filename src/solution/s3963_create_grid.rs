pub struct Solution {}

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut result = vec![];
        result.push(String::from(".").repeat(n as usize));
        for _ in 1..m {
            result.push(String::from("#").repeat(n as usize - 1) + ".");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![String::from("..."),String::from("##."), String::from("##.")],
            Solution::create_grid(3,3)
        );
    }

}
