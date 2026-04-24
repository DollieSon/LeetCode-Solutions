struct Solution{

}
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut max_dist:i32 = 0;
        let mut p1 = 0;
        while p1 < colors.len()-1 {
            let mut p2 = colors.len()-1;
            let bound = p1 + max_dist as usize;
            let item1 = colors[p1];
            while p2 > bound && p2 < colors.len(){
                let item2 = colors[p2];
                if item1 != item2 {
                    // calculate dist
                    let dist = p1.abs_diff(p2) as i32;
                    if dist > max_dist{
                        max_dist = dist;
                    }
                }
                p2 -=1;
            }
            p1 +=1;
        }
        return max_dist;
    }
}