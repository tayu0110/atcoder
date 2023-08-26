use proconio::*;

fn main() {
    input! {mut n: usize, k: usize};

    let mut s = vec![0; 60];
    let mut t = vec![0; 60];
    s[0] = 1;
    t[0] = 1;
    let mut size = 0;
    while t[size] <= n / k {
        size += 1;
        t[size] = t[size - 1] * k;
        s[size] = s[size - 1] * (k - 1);
    }

    let mut res = 0;
    for i in (0..=size).rev() {
        let u = n / t[i];
        res += u * s[i];
        n %= t[i];
        if u == k - 1 {
            n = 0;
        }
    }

    println!("{}", res)
}
