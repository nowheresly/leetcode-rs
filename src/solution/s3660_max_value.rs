

pub struct Solution {}

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 { return vec![]; }

        let mut res = vec![0; n];

        // Stack now stores: (max_val, min_val, start_idx, end_idx)
        let mut stack: Vec<(i32, i32, usize, usize)> = vec![];

        for i in 0..n {
            // Initialize current interval with both max and min as nums[i]
            let mut curr = (nums[i], nums[i], i, i);

            while let Some(top) = stack.last() {
                // If the maximum of the component on the left is greater than
                // the minimum of our current component, a jump is possible!
                if top.0 > curr.1 {
                    let prev = stack.pop().unwrap();
                    curr = (
                        curr.0.max(prev.0), // Update max of the merged component
                        curr.1.min(prev.1), // Update min of the merged component
                        prev.2,             // Start index extends to the left
                        curr.3,             // End index remains the rightmost
                    );
                } else {
                    break;
                }
            }
            stack.push(curr);
        }

        // Unpack the stack to populate the result array
        for &(max_val, _, start, end) in &stack {
            for j in start..=end {
                res[j] = max_val;
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
            vec![2,2,3],
            Solution::max_value(vec![2,1,3])
        );
    }

}
