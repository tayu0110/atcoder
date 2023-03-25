#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut f = 0;
    let mut t = 0;
    let mut o = 0;
    for v in a {
        if v % 4 == 0 {
            f += 1;
        } else if v % 2 == 1 {
            o += 1;
        } else {
            t += 1;
        }
    }

    if o > f+1 {
        println!("No");
    } else if o == f + 1 && t > 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
