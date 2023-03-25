#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};

    let mut res = (1..=n)
            .take_while(|v| *v * *v <= n)
            .filter(|v| n % *v == 0)
            .fold(vec![], |mut v, s| {
                v.push(s);
                if s * s != n {
                    v.push(n / s);
                }
                v
            });
    res.sort();
    
    for v in res {
        println!("{}", v);
    }
}
