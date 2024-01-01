use proconio::*;

fn main() {
    input! {n: usize, mut k: usize, a: [usize; n]}

    let mut now = 0;
    let mut res = 0;
    let mut used = vec![usize::MAX; n];
    let mut perm = vec![];
    let mut cnt = 0;
    while k > 0 {
        k -= 1;
        perm.push(now);
        used[now] = cnt;
        cnt += 1;
        res += a[now];
        now = res % n;
        if used[now] < usize::MAX {
            break;
        }
    }

    if k == 0 {
        println!("{res}");
        return;
    }

    let p = used[now];
    let sum = perm[p..].iter().map(|&i| a[i]).sum::<usize>();
    res += sum * (k / (perm.len() - p));
    k %= perm.len() - p;
    while k > 0 {
        k -= 1;
        res += a[now];
        now = res % n;
    }

    println!("{res}")
}
