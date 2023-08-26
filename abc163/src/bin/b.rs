use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let sum = a.iter().sum::<usize>();
    if sum <= n {
        println!("{}", n - sum)
    } else {
        println!("-1")
    }
}
