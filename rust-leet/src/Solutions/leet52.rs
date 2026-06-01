use macros::sol_macro;

sol_macro!();

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        return s.split_ascii_whitespace().last().unwrap().len() as i32;
    }
}