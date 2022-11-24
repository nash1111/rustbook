impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let nums_length = nums.len();
        let mut vec = vec![0; nums_length];
        vec[0] = nums[0];
        for i in 0..nums_length{
            let mut x = 0;
            for j in 0..i {
                x = x+nums[j];
            }
            println!{"{}",x}
            vec[i] = x+nums[i];
        }
        return vec;
    }
}

struct Solution {
}

fn main() {
    Solution::running_sum(vec![1,2,3,4]);
}