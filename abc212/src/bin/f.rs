#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, q: usize, p: [(usize, usize, usize, usize); m], q: [(usize, usize, usize); q]}

    let mut bus = vec![vec![]; n+1];
    for (i, (a, b, s, t)) in p.iter().enumerate() {
        bus[*a].push((*s, *t, *b, i));
    }
    for v in bus.iter_mut() {
        v.sort();
    }

    let mut doubling = vec![vec![std::usize::MAX; 20]; m];
    for (i, (_, b, _, t)) in p.iter().enumerate() {
        let (mut l, mut r) = (-1 as i64, bus[*b].len() as i64);
        while r - l > 1 {
            let m = (r + l) / 2;
            let (ts, _, _, _) = bus[*b][m as usize];
            if *t <= ts {
                r = m;
            } else {
                l = m;
            }
        }

        if r < bus[*b].len() as i64 {
            doubling[i][0] = bus[*b][r as usize].3;
        }
    }

    for log in 1..20 {
        for i in 0..m {
            let to = doubling[i][log-1];
            if to != std::usize::MAX {
                doubling[i][log] = doubling[to][log-1];
            }
        }
    }

    for (x, y, z) in q {
        let (mut l, mut r) = (-1 as i64, bus[y].len() as i64);
        while r - l > 1 {
            let m = (r + l) / 2;
            let (ts, _, _, _) = bus[y][m as usize];
            if x <= ts {
                r = m;
            } else {
                l = m;
            }
        }

        if r as usize == bus[y].len() {
            println!("{}", y);
            continue;
        }

        let (_, _, _, mut now) = bus[y][r as usize];
        for log in (0..20).rev() {
            let to = doubling[now][log];
            if to == std::usize::MAX {
                continue;
            }
            let (_, _, s, _) = p[to];
            if z <= s {
                continue;
            }

            now = to;
        }

        let (a, b, s, t) = p[now];
        if s < z && z <= t {
            println!("{} {}", a, b);
        } else if z <= s {
            println!("{}", a);
        } else {
            println!("{}", b);
        }
    }
}
