use macros::sol_macro;

sol_macro!();

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        if cost.len() < 3 {
            return cost.iter().sum();
        }
        let mut arr = cost.clone();
        arr.sort();
        let mut total = 0;
        for (ind,x) in arr.iter().rev().enumerate(){
            if (ind+1)%3 == 0 {
                continue;
            }
            total+=x;
        }
        return total;       
    }
}