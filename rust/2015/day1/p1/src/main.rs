use std::fs;
fn main() {
    // read in input.txt
    let input = fs::read_to_string("../input.txt").unwrap();
    //parse input.txt contents
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }
    println!("{floor}");
}
