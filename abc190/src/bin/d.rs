#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    let mut res = 0;
    let n = n * 2;
    for i in (1..=n).take_while(|i| *i * *i <= n) {
        if n % i == 0 {
            let (x, y) = (i, n / i);
            if y - 1 >= x && (y + x - 1) % 2 == 0 {
                res += 2;
            }
        }
    }

    println!("{}", res);
}
