#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {a: usize, b: usize}

        if a > b {
            res.push(a-b);
            // println!("{}", b - a);
            continue;
        }

        if b % a == 0 {
            res.push(0);
            // println!("0");
            continue;
        }

        let mut k = b / a + 1;
        let mut x = 0;
        let mut r = k*a - b;
        while k > 1 {
            k -= 1;
            let na = (b + k - 1) / k;
            if na < a {
                break;
            }
            let nx = na - a;
            if nx < x {
                break;
            }
            // eprintln!("a: {}, b: {}, k: {}, nx: {}", a, b, k, nx);
            r = std::cmp::min(r, k * (a + nx) - b + nx);
            x = nx;
        }

        res.push(r);
    }

    for v in res {
        println!("{}", v);
    }
}
