use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    let mut t = vec![];
    let (mut r, mut c) = (0i32, 0i32);
    t.push((r, c));
    for s in s {
        match s {
            'L' => c -= 1,
            'R' => c += 1,
            'U' => r -= 1,
            'D' => r += 1,
            _ => {}
        }
        t.push((r, c));
    }

    t.sort_unstable();
    t.dedup();

    if t.len() == n + 1 {
        println!("No")
    } else {
        println!("Yes")
    }
}
