use std::i32;

pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let mut streak = 0;
    let mut prev = (i32::MIN,i32::MIN);
    for x in 0..nums.len() - k as usize{
        if nums[x] > prev.0 && nums[x+k as usize] > prev.1{
            streak+=1;
        }else{ 
            prev = (i32::MIN,i32::MIN);
            streak = 1;
        }
        if streak == k{
            return true;
        }
        prev = (nums[x], nums[x+k as usize]);
    }
    return false;
}