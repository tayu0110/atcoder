#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};

    let mut d = vec![0; 100010];
    for v in a {
        d[v] += 1;
        d[v+1] += 1;
        if v > 0 {
            d[v-1] += 1;
        }
    }

    println!("{}", d.iter().max().unwrap());
}
