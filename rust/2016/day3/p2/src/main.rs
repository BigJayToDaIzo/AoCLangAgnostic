use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut tri_candidates: Vec<TriangleCandidate> = Vec::new();
    // we must pull 3 triangles/lines at a time since we need to parse COLUMNS
    let mut col_a: Vec<&str> = Vec::new();
    let mut col_b: Vec<&str> = Vec::new();
    let mut col_c: Vec<&str> = Vec::new();

    for line in input.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        col_a.push(cols[0]);
        col_b.push(cols[1]);
        col_c.push(cols[2]);
    }
    for tri_candidate in col_a.chunks(3) {
        push_triangle(tri_candidate, &mut tri_candidates);
    }
    for tri_candidate in col_b.chunks(3) {
        push_triangle(tri_candidate, &mut tri_candidates);
    }
    for tri_candidate in col_c.chunks(3) {
        push_triangle(tri_candidate, &mut tri_candidates);
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
fn push_triangle(tc: &[&str], tri_candidates: &mut Vec<TriangleCandidate>) {
    tri_candidates.push(TriangleCandidate {
        a: tc[0].trim().parse().unwrap(),
        b: tc[1].trim().parse().unwrap(),
        c: tc[2].trim().parse().unwrap(),
    })
}
#[derive(Debug)]
struct TriangleCandidate {
    a: u32,
    b: u32,
    c: u32,
}
