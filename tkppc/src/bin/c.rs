use proconio::*;

fn main() {
    input! {n: usize, m: i64, s: usize, p: [(usize, i64); n]}

    let mut time = vec![0; 100010];
    for (t, k) in p {
        time[t] += k;
    }

    let mut res = 0;
    for i in 0..s {
        time[i + 1] += time[i];
        if time[i] >= m {
            res += 1;
        }
    }

    println!("{}", res)
}
