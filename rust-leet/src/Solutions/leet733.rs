struct Solution{

}
struct coords{
    x : usize,
    y : usize,
}
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut res = image.clone();
        let mut checked = vec![vec![false;image[0].len()];image.len()];
        let mut queue : Vec<coords> = Vec::new();
        queue.push(coords { x: sr as usize, y: sc as usize });
        let thing = image[sr as usize][sc as usize];
        while let Some(c) = queue.pop() {
            res[c.x][c.y] = color; 
            // check for the adjacent
            // +1 , -1
            let dirs = [
            [1,0],[-1,0],[0,1],[0,-1]
            ];
            for mults in dirs {
                let mut new_coords = [c.x as i32, c.y as i32];
                new_coords[0] += mults[0];
                new_coords[1] += mults[1];
                if new_coords[0] >= 0 && new_coords[0] < image.len() as i32 && new_coords[1] >= 0 && new_coords[1] < image[0].len() as i32{
                    let target = coords { x: new_coords[0] as usize, y: new_coords[1] as usize };
                    println!("{}, {}", target.x,target.y);
                    if checked[target.x][target.y] == false && res[target.x][target.y] == thing{
                        checked[target.x][target.y] = true;
                        queue.push(target);
                    }
                }
            }
        }
        return res;
    }
}

