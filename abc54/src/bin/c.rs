use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut res = vec![vec![0; 1 << n]; n];
    res[0][1] = 1;
    for i in 1..1 << n {
        for j in 0..n {
            if i & (1 << j) != 0 {
                for &to in &t[j] {
                    if i & (1 << to) == 0 {
                        let next = i | (1 << to);
                        res[to][next] += res[j][i];
                    }
                }
            }
        }
    }

    let mut r = 0;
    for res in res {
        r += res[(1 << n) - 1];
    }

    println!("{}", r)
}
