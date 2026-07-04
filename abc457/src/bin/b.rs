use proconio::*;

fn main() {
    input! {n: usize}
    let mut a = vec![];
    for _ in 0..n {
        input! {l: usize, na: [usize; l]}
        a.push(na);
    }
    input! {x: usize, y: usize}
    println!("{}", a[x - 1][y - 1])
}
