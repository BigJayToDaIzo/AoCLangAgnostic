use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut tri_candidates: Vec<TriangleCandidate> = Vec::new();
    for line in input.lines() {
        let sides: Vec<&str> = line.split_whitespace().collect();
        tri_candidates.push(TriangleCandidate {
            a: sides[0].trim().parse().unwrap(),
            b: sides[1].trim().parse().unwrap(),
            c: sides[2].trim().parse().unwrap(),
        });
    }
    let mut good_triangles = 0;
    for candidate in &tri_candidates {
        if is_good_triangle(candidate) {
            good_triangles += 1;
        }
    }
    dbg!(good_triangles);
}

fn is_good_triangle(t: &TriangleCandidate) -> bool {
    t.a + t.b > t.c && t.b + t.c > t.a && t.a + t.c > t.b
}

#[derive(Debug)]
struct TriangleCandidate {
    a: u32,
    b: u32,
    c: u32,
}
