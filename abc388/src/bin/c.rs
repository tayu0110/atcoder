use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0usize;
    for &na in &a {
        let pos = a.partition_point(|&a| a < na * 2);
        res += n - pos;
    }
    println!("{res}")
}
