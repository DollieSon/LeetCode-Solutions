use macros::sol_macro;
sol_macro!();

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // b a l l o o n - b a l o n
        let mut arr = vec![0;5];
        for ch in text.chars() {
            match ch { 
                'b' => {
                    arr[0]+=1;
                },
                'a' => {
                    arr[1]+=1;
                },
                'l' => {
                    arr[2]+=1;
                },
                'o' => {
                    arr[3]+=1;
                },
                'n' => {
                    arr[4]+=1;
                },
                _ => {
                    // None
                }
            }
        }
        arr[2] = arr[2]/2;
        arr[3] = arr[3]/2;
        let thing = arr.iter().min().unwrap();
        return  *thing;
    }
}