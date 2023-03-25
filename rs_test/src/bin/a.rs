#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, k: usize};

    if k == 0 {
        println!("{}", n * n);
        std::process::exit(0);
    }

    let mut res = 0;
    for b in k+1..=n {
        res += n / b * (b - k);
        // eprintln!("b: {}, res: {}", b, res);
        let rem = n % b;
        if rem >= k {
            res += rem - k + 1;
        }

        // eprintln!("b: {}, res: {}", b, res);
    }

    println!("{}", res);
}

