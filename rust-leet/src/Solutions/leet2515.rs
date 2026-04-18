struct Solution{

}
impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let compressed: Vec<i32>= words.iter().enumerate().map(|(ind,x)| {
            if *x == target{
                let forward = ((ind as i32) - start_index).abs();
                let backward = (ind as i32).min(start_index) + (words.len() as i32 - (ind as i32 ).max(start_index));
                println!("forward = {} backward = {}",forward,backward);
                return forward.min(backward);
            }
            return -1
        }).filter(|x| *x >= 0).collect();
        println!("{:?}",compressed);
        if compressed.len() > 0 {
            return *compressed.iter().min().unwrap();
        }
        return -1;
    }
}