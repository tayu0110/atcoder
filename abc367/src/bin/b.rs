use proconio::*;

fn main() {
    input! {x: String}
    println!("{}", x.trim_end_matches('0').trim_end_matches('.'));
}
