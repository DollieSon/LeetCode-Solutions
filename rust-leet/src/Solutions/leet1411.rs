pub struct Solution{

}
impl Solution{
    const CIEL: i64 = 1_000_000_007;
    pub fn num_of_ways(n: i32) -> i32 {
        if n == 1 {
            return 12;
        }
        let mut prev = (6,6);
        for _ in 0..(n-1){
            let curr_a = ((prev.0 * 3) + (prev.1 * 2)) % Self::CIEL;
            let curr_b = ((prev.0 * 2) + (prev.1 * 2)) % Self::CIEL;
            prev = (curr_a,curr_b);
        }
        return ((prev.0 + prev.1) % Self::CIEL) as i32;
    }
}
