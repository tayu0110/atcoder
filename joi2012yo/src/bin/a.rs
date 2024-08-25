use proconio::*;

fn main() {
    input! {mut p: [usize; 3], mut j: [usize; 2]}

    p.sort();
    j.sort();
    println!("{}", p[0] + j[0] - 50)
}
