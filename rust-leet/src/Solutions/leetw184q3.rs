use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn max_total(nums: Vec<i32>, s: String) -> i64 {
        let mut curr_s = s.clone();
        for ind in 1..nums.len(){
            let ch = curr_s.as_bytes()[ind] as char;
            let prev_ch = curr_s.as_bytes()[ind-1] as char;
            if ch == '1' && prev_ch == '0'{
                let num = nums[ind];
                let prev_num = nums[ind-1];
                if prev_num > num{
                    unsafe{
                        curr_s.as_mut_vec()[ind] = b'0';
                        curr_s.as_mut_vec()[ind-1] = b'1';
                    }
                }
            }
        }
        let mut total= 0;
        for (ind,ch) in curr_s.char_indices(){
            if ch == '1'{
                total += nums[ind] as i64;
            }
        }
        return total;
    }
}


// This is Incorrect