#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, d: usize, x: [[i32; d]; n]};

    let mut res = 0;
    for i in 0..n {
        for j in i+1..n {
            let sum = x[i].iter().zip(x[j].iter()).fold(0, |s, (x, y)| s + (*x - *y) * (*x - *y));

            let f = (0..=sum).take_while(|v| *v * *v <= sum).fold(false, |f, v| f | (v * v == sum));
            if f {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
