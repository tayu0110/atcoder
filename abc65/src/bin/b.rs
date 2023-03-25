use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut now = 0;
    let mut used = vec![false; n];
    let mut cnt = 0;
    while !used[now] {
        used[now] = true;
        now = a[now] - 1;
        cnt += 1;
        if now == 1 {
            println!("{}", cnt);
            return;
        }
    }

    println!("-1")
}
