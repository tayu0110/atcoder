use proconio::input;

fn main() {
    input! {n: usize, m: usize}

    let mut p = vec![1usize];
    for _ in 0..n {
        let pp = *p.last().unwrap();
        p.push(pp * n % m);
    }

    let mut res = 0;
    let mut perm = 1;
    for l in 1..=n {
        res += perm * p[n - l] % m * ((l * (l - 1) / 2) % m) % m;
        res %= m;
        perm = perm * (n - l) % m;
    }

    res *= n;
    res %= m;

    println!("{}", res);
}
