use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut point = vec![0; n];
    for i in 0..m {
        input! {b: [usize; n]}
        let mut x = 0;
        for j in 0..n {
            point[j] += (b[j] == a[i]) as usize;
            x += (b[j] != a[i]) as usize;
        }
        point[a[i] - 1] += x;
    }

    println!("{}", point.iter().join("\n"))
}
