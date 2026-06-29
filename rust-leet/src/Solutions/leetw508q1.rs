use std::{cmp::max};

use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, mul: i32) -> i64 {
        let mut real_nums = nums.clone();
        real_nums.sort();
        let mut running_m = mul;
        let mut run_k = k;
        let mut res = 0;
        println!("{:?}",real_nums);
        for x in real_nums.iter().rev(){
            if run_k <= 0 {break;}
            let times = (*x as i64) * running_m as i64;
            let maxim = max(times, *x as i64);
            res += maxim as i64;
            print!("{maxim}");
            running_m -=1;
            run_k -=1;
        }
        return res;
    }
}