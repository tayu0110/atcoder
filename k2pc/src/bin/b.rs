#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [usize; 10], c: [Chars; n]}

    let mut max = [0; 7];
    let mut now = [0; 7];
    for v in c {
        for (i, c) in v.into_iter().enumerate() {
            if c == 'X' {
                now[i] += 1;
            } else {
                now[i] = 0;
            }
            max[i] = std::cmp::max(max[i], now[i]);
        }
    }

    a.sort();
    max.sort();

    let (mut l, mut r) = (0, 0);
    while l < 10 && r < 7 {
        if max[r] <= a[l] {
            l += 1;
            r += 1;
        } else {
            l += 1;
        }
    }

    if r == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
