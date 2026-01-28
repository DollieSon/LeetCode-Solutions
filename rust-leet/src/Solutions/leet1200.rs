use std::os::windows;

struct Solution{

}

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        // sort
        let mut thing = arr.clone();
        thing.sort();
        // find min
        let mut min_diff = thing[1] - thing[0];
        // return       
        let mut res = Vec::new();
        for x in thing.windows(2){
            //get diff
            let diff = x[1] - x[0];
            if diff < min_diff{
                min_diff = diff;
                res = Vec::new();
            }
            if diff == min_diff{
                let temp = vec![x[0],x[1]];
                res.push(temp);
            }
        }
        return res;
    }
}