#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize};

    if n % 2 == 1 {
        println!("0");
        std::process::exit(0);
    }

    n /= 2;
    let mut res = 0usize;
    for p in 1..=40 {
        let pow = 5usize.pow(p);
        res += n / pow;

        if std::usize::MAX / pow < 5 {
            break;
        }
    }

    println!("{}", res);
}
