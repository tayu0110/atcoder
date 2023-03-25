use num::integer::Roots;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize, k: usize};
    let mut res = vec![];
    let mx = n.sqrt() + 2;
    for i in 2..mx {
        while n % i == 0 {
            res.push(i);
            n /= i;
        }
    }
    if n != 1 {
        res.push(n);
    }

    if res.len() < k {
        println!("-1");
    } else {
        while res.len() > k {
            let (b, bb) = (res.pop().unwrap(), res.pop().unwrap());
            res.push(b * bb);
        }
        for i in 0..k {
            if i > 0 {
                print!(" ");
            }
            print!("{}", res[i]);
        }
        println!("");
    }
}
