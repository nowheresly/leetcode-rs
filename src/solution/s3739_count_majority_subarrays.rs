pub struct Solution {}

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        let mut res = 0i64;

        let mut arr:Vec<i32> = vec![0; n];
        for i in 0..n {
            arr[i] = if nums[i] == target { 1 } else { -1 };
        }
        let mut prefix = vec![0; n+1];
        for i in 0..n {
            prefix[i+1] = prefix[i]+ arr[i];
        }

        let offset = n as i32 + 2;
        let bit_size = 2 * n + 3;
        let mut bit = vec![0; bit_size];

        for p in prefix {
            res += query(&mut bit, p + offset-1);

            // Add the current prefix sum to our Fenwick Tree
            update(&mut bit, bit_size, p+offset, 1);
        }
        res
    }
}

fn update(bit: &mut Vec<i32>, n: usize, i: i32, delta: i32) {
    let mut i = i as i32;
    let n = n as i32;
    while i < n {
        bit[i as usize] += delta;
        i += i & (-i);
    }
}

fn query(bit: &mut Vec<i32>, i: i32) -> i64 {
    let mut i = i as i32;
    let mut sum = 0i64;
    while i > 0 {
        sum += bit[i as usize] as i64;
        i -= i & (-i);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2));
    }
}
