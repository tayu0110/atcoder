use proconio::*;

fn main() {
    input! {n: usize, k: usize}
    let mut res = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j < k && k <= i + j + n {
                res += 1;
            }
        }
    }
    println!("{}", res)
}
