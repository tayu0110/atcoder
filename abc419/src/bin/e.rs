use proconio::*;

fn main() {
    input! {n: usize, m: usize, l: usize, mut a: [usize; n]}
    a.insert(0, 0);
    for i in 0..n {
        a[i + 1] += a[i];
        a[i + 1] %= m;
    }

    eprintln!("a: {a:?}");
    let mut res = 0;
    let mut base = 0;
    for i in l..=n {
        let t = (a[i] + base) % m;
        let rem = if t <= a[i - l] {
            a[i - l] - t
        } else {
            m - (t - a[i - l])
        };
        res += rem;
        base += rem;
        eprintln!("l: {i}, res: {res}, base: {base}");
    }

    println!("{res}")
}
