impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // target_index: 0~length
        // 
        let nums_length = nums.len();
        for index in 0..nums_length {
            let mut left_side = 0;
            let mut right_side = 0;
            for left_index in 0..index {
                left_side = left_side + nums[left_index];
            }
            for right_index in index+1..nums_length {
                right_side = right_side + nums[right_index];
            }
            if left_side == right_side {
                return index as i32;
            }
        }
        return -1;
    }
}

struct Solution {
}

fn main() {
    Solution::pivot_index(vec![1,2,3,4]);
}