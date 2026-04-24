use std::i32;

struct Solution{

}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max_dist:i32 = -1;
        for num in nums.iter(){
            if *num > min && (*num).abs_diff(min) as i32> max_dist{
                max_dist = (*num).abs_diff(min) as i32;
            }
            if *num < min {
                min = *num;
            }
        }
        return max_dist;
    }
}