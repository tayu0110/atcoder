use proconio::*;

fn dfs(now: usize, par: usize, colors: &mut [u8], s: &[u8], t: &Vec<Vec<usize>>) -> bool {
    let (mut b, mut w) = (0, 0);
    let mut res = true;
    for &to in &t[now] {
        if to != par {
            res &= dfs(to, now, colors, s, t);
            if colors[to] == b'B' {
                b += 1;
            } else {
                w += 1;
            }
        }
    }

    if b == w {
        res &= !(colors[par] != 0 && colors[par] != s[now]);
        colors[par] = s[now];
    } else if b < w {
        res &= !(s[now] == b'B');
    } else {
        res &= !(s[now] == b'W');
    }

    if colors[now] == 0 {
        colors[now] = s[par];
    }

    res
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, p: [(usize, usize); n-1], s: marker::Bytes}

        let mut t = vec![vec![]; n];
        for (a, b) in p {
            t[a - 1].push(b - 1);
            t[b - 1].push(a - 1);
        }

        let mut colors = vec![0; n];
        if dfs(0, 0, &mut colors, &s, &t) {
            println!(
                "{}",
                colors.into_iter().map(|c| c as char).collect::<String>()
            )
        } else {
            println!("-1")
        }
    }
}
