use proconio::*;

fn rec(n: usize, val: usize, now: usize, a: &[usize], res: &mut usize) {
    if val > n {
        return;
    }
    if now == 0 {
        *res += 1;
        return;
    }

    if val % a[now - 1] != 0 {
        rec(n, val, now - 1, a, res);
    }
    rec(n, val * a[now - 1], now - 1, a, res);
}

fn main() {
    input! {n: usize, q: usize, a: [usize; q]}

    let mut res = 0;
    rec(n, 1, q, &a, &mut res);
    println!("{res}")
}
