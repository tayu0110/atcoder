use proconio::*;

fn main() {
    input! {mut n: usize}

    let mut f = vec![0, 1];
    while f.last().unwrap() < &n {
        f.push(f[f.len() - 1] + f[f.len() - 2]);
    }

    let mut cnt = 0;
    while let Some(now) = f.pop() {
        if 0 < now && now <= n {
            cnt += n / now;
            n %= now;
        }
    }

    println!("{}", cnt)
}
