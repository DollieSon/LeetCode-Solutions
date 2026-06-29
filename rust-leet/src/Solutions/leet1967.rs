use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let x = patterns.iter().map(|s| word.contains(s)).filter(|x| *x == true).count();
        return x as i32;       
    }
}