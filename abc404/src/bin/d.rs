use proconio::*;

fn main() {
    input! {n: usize, m: usize, c: [usize; n]}

    let mut park = vec![vec![]; n];
    for i in 0..m {
        input! {k: usize, a: [usize; k]}
        for a in a {
            park[a - 1].push(i);
        }
    }

    let mut res = usize::MAX;
    for i in 1..1 << (2 * n) {
        let mut t = vec![0; m];
        let mut sum = 0;
        for j in 0..n {
            let cnt = (i >> (2 * j)) & 0b11;
            if cnt > 0 {
                for k in 0..park[j].len() {
                    t[park[j][k]] += cnt;
                }
                sum += cnt * c[j];
            }
        }

        if t.iter().all(|&t| t >= 2) {
            res = res.min(sum);
        }
    }

    println!("{}", res);
}
