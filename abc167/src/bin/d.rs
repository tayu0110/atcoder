use proconio::*;

fn main() {
    input! {n: usize, mut k: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|a| *a -= 1);

    let mut per = vec![std::usize::MAX; n];
    let mut now = 0;
    let mut cnt = 0;
    while per[now] == std::usize::MAX {
        per[now] = cnt;
        cnt += 1;
        now = a[now];
    }

    if k < cnt {
        println!("{}", per.iter().position(|&p| p == k).unwrap() + 1);
        return;
    }

    k -= per[now];
    k %= cnt - per[now];

    println!(
        "{}",
        per.iter().position(|&p| p == k + per[now]).unwrap() + 1
    )
}
