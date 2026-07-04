use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    for i in 0..n {
        if let Some(pos) = a[..i].iter().rposition(|&b| b > a[i]) {
            println!("{}", pos + 1)
        } else {
            println!("-1")
        }
    }
}
