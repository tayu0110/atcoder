#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {mut a: i32, mut b: i32, mut c: i32}

    for i in 2..=100 {
        let (na, nb, nc) = (b-c, c-a, a-b);

        if na == 0 || nb == 0 || nc == 0 {
            println!("{}", i);
            std::process::exit(0);
        }

        a = na;
        b = nb;
        c = nc;
    }

    println!("-1");
}
