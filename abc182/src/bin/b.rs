#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};

    let mut res = 0;
    let mut max = 0;
    for i in 2..1010 {
        let mut cnt = 0;
        for j in 0..n {
            if a[j] % i == 0 {
                cnt += 1;
            }
        }

        if cnt > max {
            max = cnt;
            res = i;
        }
    }

    println!("{}", res);
}
