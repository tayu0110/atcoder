#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, k: usize};

    if k * 2 > n {
        println!("-1");
        std::process::exit(0);
    }

    let mut res = vec![];
    let mut now = 0;
    while now + k * 4 <= n {
        for i in now..now+k {
            res.push(i+k);
        }
        for i in now+k..now+k*2 {
            res.push(i-k);
        }
        now += k * 2;
    }

    if n - now <= k * 3 {
        for i in now..n-k {
            res.push(i + k);
        }
        for i in n-k..n {
            res.push(i - (n-k) + now);
        }
    } else {
        for i in now..now+k {
            res.push(i + k);
        }
        for i in now+k..n-k*2 {
            res.push(i - k);
        }
        for i in n-k*2..n-k {
            res.push(i + k);
        }
        for i in n-k..now+k*3 {
            res.push(i - k * 2);
        }
        for i in now+k*3..n {
            res.push(i - k);
        }
    }

    for (i, v) in res.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v+1);
    }
    println!();
}
