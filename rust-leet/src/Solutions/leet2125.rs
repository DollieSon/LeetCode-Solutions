pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let compressed:Vec<i32> = bank.iter().map(|x| x.chars().filter(|ch| *ch == '1').count() as i32).collect();
    println!("{:?}",compressed);
    let mut total = 0;
    let mut prev = compressed[0];
    for x  in 1..compressed.len(){
        let item = compressed[x];
        if item == 0{continue;}
        total += prev * item;
        prev = item;
    }
    return total;
}