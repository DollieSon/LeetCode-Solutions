use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return image.iter().map(|line| {
            line.iter().copied().map(|f| match f {
                0 => 1,
                1 => 0,
                _ => 2,
            }).rev().collect()
        }).collect();
    }
}