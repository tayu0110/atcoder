use proconio::*;

fn main() {
    input! {n: usize, mut h: i32, mut m: i32, mut p: [(i32, i32); n]}

    if h > m {
        (h, m) = (m, h);
        p.iter_mut()
            .for_each(|v| std::mem::swap(&mut v.0, &mut v.1));
    }

    let mut memo = vec![-1; h as usize + 1];
    memo[h as usize] = m;
    for (i, (a, b)) in p.into_iter().enumerate() {
        let mut nt = vec![-1; h as usize + 1];
        for (h, &m) in memo.iter().enumerate() {
            if h as i32 >= a && m >= 0 {
                nt[h - a as usize] = nt[h - a as usize].max(m);
            }

            if m >= b {
                nt[h] = nt[h].max(m - b);
            }
        }

        memo = nt;
        if memo.iter().all(|&nt| nt < 0) {
            println!("{i}");
            return;
        }
    }

    println!("{n}")
}
