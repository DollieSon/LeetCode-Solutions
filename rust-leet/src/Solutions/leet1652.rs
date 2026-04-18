struct Solution{

}
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![0;code.len()];
        }
        let data: Vec<i32> = if k < 0 {
            code.iter().cycle().skip(code.len() - k.abs() as usize).take(code.len() + k.abs() as usize - 1).cloned().collect()
        }else{
            code.iter().cycle().skip(1).take(code.len() + k as usize -1).cloned().collect()
        };
        println!("{:?}",data);
        let mut res = Vec::new();
        let mut running_count = 0;
        let mut tail_val=  0;
        for x in data.windows(k.abs() as usize){
            if res.len() == 0 {
                running_count = x.iter().sum();
                tail_val = x[0];
            }else{
                running_count += x[x.len()-1];
                running_count -= tail_val
            }
            println!("rc {}", running_count);
            res.push(running_count);
            tail_val = x[0];
        }
        return res;
    }
}