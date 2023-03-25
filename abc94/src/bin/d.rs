#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]}

    let max = *a.iter().max().unwrap();
    let mut j = 0;
    for i in 0..n {
        if a[i] == max {
            j = i;
        }
    }

    let mut res = 0;
    let mut idx = 0;
    for (i, v) in a.iter().enumerate() {
        if i == j {
            continue;
        }
        let v = std::cmp::min(*v, max-*v);
        if res <= v {
            res = v;
            idx = i;
        }
    }

    println!("{} {}", max, a[idx]);
}
