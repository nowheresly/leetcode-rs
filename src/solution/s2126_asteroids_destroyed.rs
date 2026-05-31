pub struct Solution {}

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        asteroids.sort();
        let mut mass:i64 = mass as i64;
        for i in 0..asteroids.len() {
            if mass < asteroids[i] as i64 {
                return false;
            }
            mass += asteroids[i] as i64;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::asteroids_destroyed(10, vec![3,9,19,5,21]));

    }
}
