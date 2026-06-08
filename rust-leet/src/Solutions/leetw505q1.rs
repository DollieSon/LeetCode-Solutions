use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn sum_of_good_integers(n: i32, k: i32) -> i32 {
        let mut total = 0;
        for x in std::cmp::max(0,k-n)..n+k{
            if (x & n) == 0{
                total += x;
            }
        }
        return 0;      
    }
}