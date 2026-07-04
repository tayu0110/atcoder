use proconio::*;

fn rec(n: usize, a: &[u32], s: &mut [u32], memo: &mut [u8]) -> u32 {
    if a.is_empty() {
        let max = s[..n].iter().max().unwrap();
        let min = s[..n].iter().min().unwrap();
        return max - min;
    }

    let mut res = u32::MAX;
    for i in 0..n {
        if memo[i] < 3 {
            memo[i] += 1;
            s[i] += a[0];
            res = res.min(rec(n, &a[1..], s, memo));
            s[i] -= a[0];
            memo[i] -= 1;
            if memo[i] == 0 {
                break;
            }
        }
    }
    res
}

fn main() {
    input! {n: usize, a: [u32; 3 * n]}

    let mut memo = [0; 5];
    let mut s = [0; 5];
    println!("{}", rec(n, &a, &mut s, &mut memo))
}
