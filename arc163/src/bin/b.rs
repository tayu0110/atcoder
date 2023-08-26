use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [i64; n]}

    let mut t = a[2..].to_vec();
    t.sort();

    let mut res = std::i64::MAX;
    for i in 0..t.len() {
        if i + m - 1 >= t.len() {
            break;
        }

        let (f, s) = (t[i], t[i + m - 1]);

        let r = (a[0] - f).max(0) + (s - a[1]).max(0);
        res = res.min(r);
    }

    println!("{}", res);
}
