use proconio::*;

fn dfs(now: usize, xor: usize, count: usize, k: usize, mask: usize, a: &[usize]) -> usize {
    if count == k {
        return xor ^ mask;
    }

    let mut max = 0;
    let len = a.len();
    for next in now..len {
        if k - count > len - next {
            return max;
        }
        max = max.max(dfs(next + 1, xor ^ a[next], count + 1, k, mask, a));
    }
    max
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let nk = n - k;
    if k < nk {
        println!("{}", dfs(0, 0, 0, k, 0, &a))
    } else {
        let mask = a.iter().fold(0, |s, v| s ^ *v);
        println!("{}", dfs(0, 0, 0, nk, mask, &a))
    }
}
