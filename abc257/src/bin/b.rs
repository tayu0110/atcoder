#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, k: usize, q: usize, a: [usize; k], l: [usize; q]};

    let mut a = a;
    for v in l {
        let v = v - 1;

        let to = std::cmp::min(a[v]+1, n);
        let mut bad = false;
        for (i, w) in a.iter().enumerate() {
            if i == v {
                continue;
            }
            if *w == to {
                bad = true;
            }
        }
        if !bad {
            a[v] = to;
        }
    }

    for i in 0..k {
        if i != 0 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
    println!();
}
