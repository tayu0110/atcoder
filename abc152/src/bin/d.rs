use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 0usize;
    let mut t = vec![vec![0; 10]; 10];
    for i in 1..=n {
        let b = i % 10;
        let mut f = i;
        while f >= 10 {
            f /= 10;
        }
        res += t[f][b] * 2;
        if f == b {
            res += 1;
        }
        t[b][f] += 1;
    }

    println!("{}", res)
}
