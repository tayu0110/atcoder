#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: [usize; 2], b: [usize; 2], c: [usize; 2]};

    let mut res = 0f64;
    for now in a[0]..=a[1] {
        if now >= b[1] || now >= c[1] {
            break;
        }
        res += ((b[1] - std::cmp::max(now, b[0]-1)) * (c[1] - std::cmp::max(now, c[0]-1))) as f64 / ((a[1] - a[0] + 1) * (b[1] - b[0] + 1) * (c[1] - c[0] + 1)) as f64;
    }

    println!("{}", res);
}
