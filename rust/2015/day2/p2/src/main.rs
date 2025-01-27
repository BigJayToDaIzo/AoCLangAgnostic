use std::fs;

fn main() {
    // read in input.txt
    let input = fs::read_to_string("../input.txt").unwrap();
    // break this thing down into package dimensions
    // let's use a lines iterator!
    let package_dimensions = input.lines();
    let mut total_ribbon_len = 0;

    // now we break each line into l, w, & h
    for dimension in package_dimensions {
        // clamp into l, w, & h by splitting on x
        let d_vec: Vec<&str> = dimension.split('x').collect();
        total_ribbon_len += trl(d_vec);
    }
    dbg!(total_ribbon_len);
}

fn trl(v: Vec<&str>) -> u32 {
    let l: u32 = v[0].parse().unwrap();
    let w: u32 = v[1].parse().unwrap();
    let h: u32 = v[2].parse().unwrap();
    bow_ribbon_len(l, w, h) + l * w * h
}

fn bow_ribbon_len(l: u32, w: u32, h: u32) -> u32 {
    let mut v = [l, w, h];
    v.sort();
    2 * v[0] + 2 * v[1]
}
