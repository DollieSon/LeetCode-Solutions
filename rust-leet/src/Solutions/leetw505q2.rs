use macros::sol_macro;
sol_macro!();

impl Solution {
    // n is the len and target is points less than k
    pub fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
        // generate bits with len n without consecutive 1
        let mut s = vec![String::from("0"),String::from("1")];
        while s[0].len() < n as usize  && n != 1{
            let mut new_s = Vec::new();
            // add the 0
            for word in s{
                let mut x = String::from("0");
                x.push_str(&word);
                new_s.push(x);

                if word.chars().next().unwrap() == '0'{
                    let mut x = String::from("1");
                    x.push_str(&word);
                    new_s.push(x);
                }
            }
            s = new_s;
            println!("{:?}",s);
            
        }
        let mut res = vec![];
        for word in s {
            let mut total = 0;
            for (ind,num) in word.char_indices(){
                if num == '1'{
                    total+=ind as i32;
                }
            }
            if total <= k{
                res.push(word);
            }
        }
        return res;

    }
}