use proconio::*;

fn main() {
    input! {n: usize, mut p: [usize; n]}
    p.iter_mut().for_each(|p| *p -= 1);

    let mut ret = 0usize;
    let mut memo = vec![false; n];
    for i in 0..n {
        if memo[i] {
            continue;
        }

        let mut group = 0;
        let mut now = i;
        while !memo[now] {
            memo[now] = true;
            group += 1;
            now = p[now];
        }

        if group == 1 {
            continue;
        }

        ret += group * (group - 1) / 2;
    }

    println!("{}", ret);
}
