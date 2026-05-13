

pub struct Solution {
}

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let limit = limit as usize;
        let mut diff:Vec<i32> = vec![0; 2 * limit + 2];

        for i in 0..n/2 {
            let a = nums[i];
            let b = nums[n-1-i];

            // 1. Minimum and maximum possible sums with 1 move
            let min_sum_1_move = i32::min(a, b) + 1;
            let max_sum_1_move = i32::max(a,b) + limit as i32;

            // 2. Default: 2 moves for every pair
            // We represent this by adding 2 to the starting point of all possible sums
            diff[2] += 2;
            diff[2 * limit + 1] -= 2;

            // 3. Improve to 1 move for the range [minSum1Move, maxSum1Move]
            diff[min_sum_1_move as usize] -= 1;
            diff[max_sum_1_move as usize + 1] += 1;

            // 4. Improve to 0 moves for the exact sum a + b
            diff[a as usize + b as usize] -= 1;
            diff[a as usize + b as usize + 1] += 1;
        }
        let mut min_moves:i32 = n as i32;
        let mut current_moves = 0;

        // Sweep through the difference array to find the minimum moves
        for i in 2..=2*limit {
            current_moves += diff[i];
            min_moves = min_moves.min(current_moves);
        }
        min_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_moves(vec![1,2,4,3], 4));

    }
}
