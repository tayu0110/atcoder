use proconio::*;

fn main() {
    input! {mut n: usize, x: usize, m: usize}

    let mut found = vec![std::usize::MAX; m];
    let mut now = x;
    let mut cnt = 0;
    while found[now] == std::usize::MAX {
        found[now] = cnt;
        cnt += 1;
        now = now * now % m;
    }

    if n < found[now] {
        println!("{}", (0..m).filter(|&v| found[v] < n).sum::<usize>());
        return;
    }

    let mut res = (0..m).filter(|&v| found[v] < found[now]).sum::<usize>();
    n -= found[now];

    let cycle = (0..m)
        .filter(|&v| found[v] < std::usize::MAX && found[v] >= found[now])
        .sum::<usize>();
    res += n / (cnt - found[now]) * cycle;
    n %= cnt - found[now];

    res += (0..m)
        .filter(|&v| {
            found[v] < std::usize::MAX && found[v] >= found[now] && found[v] - found[now] < n
        })
        .sum::<usize>();

    println!("{}", res)
}
