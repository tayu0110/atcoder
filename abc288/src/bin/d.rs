use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [i64; n], q: usize, p: [(usize, usize); q]}

    let mut cum = vec![vec![0i64; n + 1]; k];
    for (i, &v) in a.iter().enumerate() {
        cum[i % k][i + 1] = v;
        for j in 0..k {
            cum[j][i + 1] += cum[j][i];
        }
    }

    for (l, r) in p {
        if (r - k + 2..=r).all(|r| {
            let rem = (r - 1) % k;
            let mut t = cum[rem][r] - cum[rem][l - 1];
            let rem = (rem + k - 1) % k;
            t -= cum[rem][r] - cum[rem][l - 1];
            t == 0
        }) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
