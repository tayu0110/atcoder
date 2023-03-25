#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [i64; n]};

    let na = a.iter().skip(1).enumerate().fold(a[0], |s, (i, v)| {
        if i % 2 == 0 {
            s - *v
        } else {
            s + *v
        }
    });

    let mut res = vec![na];
    for i in 0..n {
        res.push((a[i] - res[i] / 2) * 2);
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res[i]);
    }
    println!("");
}
