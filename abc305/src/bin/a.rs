use proconio::*;

fn main() {
    input! {n: usize}
    let k = n / 5 * 5;
    let l = k + 5;
    if n - k > l - n {
        println!("{}", l);
    } else {
        println!("{}", k)
    }
}
