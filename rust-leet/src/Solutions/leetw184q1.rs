use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn consecutive_set_bits(n: i32) -> bool {
        let bits:Vec<char> = format!("{:b}",n).chars().collect();
        let mut paired = false;
        for thing in bits.windows(2){
            if thing[0] == '1' && thing[0] == thing[1]{
                if paired == true{
                    return false;
                }
                paired = true;
            }
        }
        return false;
    }
}