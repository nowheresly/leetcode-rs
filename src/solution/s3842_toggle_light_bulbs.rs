pub struct Solution {}

impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut b = vec![false;101];
        for i in bulbs {
            b[i as usize] = !b[i as usize];
        }
        let mut res = vec![];
        for i in 0..b.len() {
            if b[i] {
                res.push(i as i32);
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
        assert_eq!(
            vec![20,30],
            Solution::toggle_light_bulbs(vec![10,30,20,10])
        );
    }

}
