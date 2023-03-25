use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [i64; n]}

    let mut res = a
        .iter()
        .take(m)
        .enumerate()
        .fold(0, |s, (i, v)| s + (i as i64 + 1) * *v);
    let mut sum = a.iter().take(m).sum::<i64>();

    let mut now = res;
    for i in m..n {
        now -= sum;
        now += m as i64 * a[i];
        sum = sum + a[i] - a[i - m];
        res = std::cmp::max(res, now);
    }

    println!("{}", res);
}
