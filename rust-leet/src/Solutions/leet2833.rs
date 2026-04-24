struct Solution{

}
impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut pos:i32 = 0;
        let mut empty:i32 = 0;
        for x in moves.chars(){
            match x {
                '_' => {
                    empty+=1;
                }
                'L' => {
                    pos -=1;
                }
                'R' => {
                    pos+=1;
                }
                _ => {
                    // none
                }
            }
        }
        if pos >= 0 {
            pos += empty;
        }else {
            pos -= empty;
        }
        return pos.abs();       
    }
}