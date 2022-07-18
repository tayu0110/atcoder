#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, k: usize};

    if k == 1 {
        let mut v = vec![0; n+1];
        for i in 1..n+1 {
            v[i] = i;
        }

        for i in (1..n).step_by(2) {
            v.swap(i, i+1);
        }

        if n % 2 == 1 {
            v.swap(n-2, n);
        }

        for i in 1..n+1 {
            if i > 1 {
                print!(" ");
            }
            print!("{}", v[i]);
        }
        println!("");
        std::process::exit(0);
    }

    if n - k + 1 < k + 1 {
        println!("-1");
        std::process::exit(0);
    }

    let mut now = k + 1;
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", now);
        now += 1;
        if now > n {
            now = 1;
        }
    }

    println!("");
}
