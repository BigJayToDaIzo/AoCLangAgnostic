use ascii_converter::*;
fn main() {
    let input = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];

    match decimals_to_string(&input) {
        Ok(num) => println!("* Output: {}", num),
        Err(e) => println!("* Woopsie: {}", e),
    };
}
