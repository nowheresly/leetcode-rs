use std::collections::{HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // 1. Find max value to size our Sieve properly
        let mut max_val = 0;
        for num in nums.iter() {
            max_val = max_val.max(*num as usize);
        }

        // 2. Build the smaééest Prime Factor (SPF) array using a Sieve
        let mut spf = vec![0; max_val + 1];
        for i in 2..=max_val {
            spf[i] = i;
        }
        let mut i = 2;
        while i * i <= max_val {
            if spf[i] == i { // i is prime
                // Replace `step i` with `.step_by(i)`
                for j in (i * i..=max_val).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
            i += 1;
        }
        // 3. Map each prime to the indices of elements that are divisible by it
        let mut prime_to_indices: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let mut temp = nums[i] as usize;
            while temp > 1 {
                let p = spf[temp];

                prime_to_indices.entry(p).or_default().push(i);

                while temp % p == 0 {
                    temp /= p;
                }
            }
        }

        // 4. Breadth-First Search (BFS) Initialization
        let mut q = VecDeque::new();
        let mut visited_indices = vec![false; n];
        let mut visited_primes = vec![false; max_val + 1];

        q.push_back(0);
        visited_indices[0] = true;
        let mut jumps = 0;

        // 5. Traverse the graph
        while !q.is_empty() {
            let level_size = q.len();

            for _ in 0..level_size {
                let curr = q.pop_front().unwrap();

                // If we reached the target index, return the jump count
                if curr == n - 1 {
                    return jumps;
                }

                // Move 1: Adjacent Step Forward
                if curr + 1 < n && !visited_indices[curr + 1] {
                    visited_indices[curr + 1] = true;
                    q.push_back(curr + 1);
                }

                // Move 2: Adjacent Step Backward
                if curr > 0 && !visited_indices[curr - 1] {
                    visited_indices[curr - 1] = true;
                    q.push_back(curr - 1);
                }

                // Move 3: Prime Teleportation
                let val = nums[curr] as usize;

                // Check if nums[curr] is prime itself
                if val > 1 && spf[val] == val {
                    if !visited_primes[val] {
                        visited_primes[val] = true; // Mark prime as used

                        // Grab all indices divisible by this prime
                        if let Some(indices) = prime_to_indices.get(&val) {
                            for &next_idx in indices {
                                if !visited_indices[next_idx] {
                                    visited_indices[next_idx] = true;
                                    q.push_back(next_idx);
                                }
                            }
                        }
                    }
                }
            }
            // Increment jumps after checking all possibilities at the current depth level
            jumps += 1;
        }

        -1 // Fallback if n-1 is unreachable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_jumps(vec![1, 2, 4, 6]));
    }
}
