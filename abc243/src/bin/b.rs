#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    println!("{}", a.iter().zip(b.iter()).filter(|(na, nb)| na == nb).count());

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if a[i] == b[j] && i != j {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
