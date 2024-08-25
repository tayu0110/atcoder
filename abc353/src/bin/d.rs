use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ten = vec![0; 15];
    ten[0] = 1;
    for i in 1..15 {
        ten[i] = ten[i - 1] * 10;
        ten[i] %= M;
    }

    let mut p = [0; 12];
    for &a in &a {
        let len = a.to_string().len();
        p[len] += 1;
    }

    let mut res = 0;
    for (i, a) in a.into_iter().enumerate() {
        let len = a.to_string().len();
        p[len] -= 1;

        for (i, &p) in p.iter().enumerate() {
            let t = a * ten[i] % M * p % M;
            res += t;
            res %= M;
        }

        res += a * i;
        res %= M;
    }

    println!("{res}")
}
