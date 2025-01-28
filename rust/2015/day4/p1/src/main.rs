fn main() {
    let input = "iwrupvqb";
    for num in 0..u32::MAX {
        let digest = md5::compute(format!("{}{}", input, num));
        let digest_str = format!("{:x}", digest);
        if &digest_str[..5] == "00000" {
            println!("{}", num);
            break;
        }
    }
}
