use proconio::*;

fn main() {
    input! {n: usize, mut t: usize, c: [usize; n], r: [usize; n]}

    if c.iter().all(|&c| c != t) {
        t = c[0];
    }
    let mut res = 0;
    let mut max = 0;
    for i in 0..n {
        if c[i] == t && max < r[i] {
            max = r[i];
            res = i + 1;
        }
    }
    println!("{}", res)
}
