use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: marker::Chars, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut bw = vec![vec![0; 2]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);

        bw[a - 1][(s[b - 1] == 'B') as usize] += 1;
        bw[b - 1][(s[a - 1] == 'B') as usize] += 1;
    }

    let mut nt = std::collections::VecDeque::new();
    let mut bad = vec![false; n];
    for i in 0..n {
        if bw[i][0] * bw[i][1] == 0 {
            bad[i] = true;
            nt.push_back(i);
        }
    }

    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            bw[to][(s[now] == 'B') as usize] -= 1;

            if bw[to][0] * bw[to][1] == 0 && !bad[to] {
                bad[to] = true;
                nt.push_back(to);
            }
        }
    }

    let res = bad.into_iter().fold(true, |s, v| s & v);
    if !res {
        println!("Yes")
    } else {
        println!("No")
    }
}
