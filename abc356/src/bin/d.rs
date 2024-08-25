use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {mut n: usize, m: usize}
    n += 1;

    let mut res = 0;
    for i in (0..60)
        .take_while(|&i| 1 << i <= n)
        .filter(|&i| (m >> i) & 1 != 0)
    {
        let k = 1 << i;
        res += n / (k * 2) * k;
        res %= M;

        let r = n % (k * 2);
        res += r.saturating_sub(k);
        res %= M;
    }

    println!("{res}")
}
