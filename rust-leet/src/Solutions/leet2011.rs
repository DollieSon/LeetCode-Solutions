pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let pos = operations.iter()
    .filter(|x| x.contains("X++") || x.contains("++X"))
    .count() as i32;
    return pos - (operations.len() as i32 - pos);
}