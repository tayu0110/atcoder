use std::{collections::HashSet, time::SystemTime};

use itertools::Itertools;
use proconio::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use unionfind::UnionFind;

const N: usize = 50;
const M: usize = 100;
const AGING_THRESHOLD: i32 = 100;

fn transpose(c: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let (n, m) = (c.len(), c[0].len());
    let mut new = vec![vec![0; n]; m];
    for i in 0..n {
        for j in 0..m {
            new[j][i] = c[i][j];
        }
    }
    new
}

fn make_adj(c: &Vec<Vec<u8>>) -> Vec<u128> {
    let mut adj = vec![0; M + 1];
    for i in 0..c.len() {
        for j in 0..c[0].len() {
            if j == 0 {
                adj[c[i][0] as usize] |= 1;
            }
            if i == 0 {
                adj[c[i][j] as usize] |= 1;
            }
            for (dx, dy) in vec![(0, 1), (1, 0)] {
                let (ni, nj) = (i.wrapping_add(dx), j.wrapping_add(dy));
                if ni < c.len() && nj < c[0].len() {
                    adj[c[i][j] as usize] |= 1 << c[ni][nj];
                    adj[c[ni][nj] as usize] |= 1 << c[i][j];
                } else {
                    adj[c[i][j] as usize] |= 1;
                    adj[0] |= 1 << c[i][j];
                }
            }
        }
    }
    adj
}

fn verify(adj: &Vec<u128>, nadj: &Vec<u128>, nc: &Vec<Vec<u8>>) -> bool {
    let (h, w) = (nc.len(), nc[0].len());
    let mut uf = UnionFind::new(h * w);
    for i in 0..h {
        for j in 0..w {
            for (dx, dy) in vec![(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let (ni, nj) = (i.wrapping_add(dx), j.wrapping_add(dy));
                if ni < h && nj < w && nc[i][j] == nc[ni][nj] {
                    uf.merge(i * w + j, ni * w + nj);
                }
            }
        }
    }

    (0..h * w).map(|i| uf.root(i)).collect::<HashSet<_>>().len() == M && adj == nadj
}

fn try_remove(c: &mut Vec<Vec<u8>>, adj: &Vec<u128>, r: usize) -> bool {
    if c.len() > 1 {
        let mut nc = c.clone();
        nc.remove(r);
        let nadj = make_adj(&nc);

        if verify(&adj, &nadj, &nc) {
            *c = nc;
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn try_remove_random(c: &mut Vec<Vec<u8>>, adj: &Vec<u128>, rng: &mut ThreadRng) -> bool {
    if c.len() > 1 {
        let r = rng.gen_range(0..c.len());
        try_remove(c, adj, r)
    } else {
        false
    }
}

fn main() {
    input! {_: usize, _: usize, c: [[u8; N]; N]}

    let adj = make_adj(&c);

    let mut rng = thread_rng();
    let tm = SystemTime::now();
    let mut score = 0;
    let mut res = c.clone();
    let mut try_count = 0;
    while tm.elapsed().unwrap().as_millis() < 1930 {
        let mut c = c.clone();

        let mut aging = AGING_THRESHOLD;
        while aging > 0 {
            if try_remove_random(&mut c, &adj, &mut rng) {
                aging = AGING_THRESHOLD;
            }

            c = transpose(c);
            aging -= 1;
        }

        if N * N - c.len() * c[0].len() > score {
            score = N * N - c.len() * c[0].len();
            res = c;
        }
        try_count += 1;
    }

    // let keep = res.clone();
    for _ in 0..4 {
        let mut i = 0;
        while i < res.len() {
            if !try_remove(&mut res, &adj, i) {
                i += 1;
            }
        }
        res = transpose(res);
    }

    let (h, w) = (res.len(), res[0].len());
    fn power_play(i: usize, j: usize, res: &mut Vec<Vec<u8>>) {
        let (h, w) = (res.len(), res[0].len());
        let mut e = vec![];
        for (dx, dy) in vec![
            (0, 1),
            (1, 0),
            (0, !0),
            (!0, 0),
            (!0, 1),
            (1, 1),
            (1, !0),
            (!0, !0),
        ] {
            let (ni, nj) = (i.wrapping_add(dx), j.wrapping_add(dy));
            if ni < h && nj < w {
                e.push(res[ni][nj]);
            } else {
                e.push(0);
            }
        }

        if e[7] == 0
            && e[3] == 0
            && e[2] == 0
            && e[0] == res[i][j]
            && e[1] == res[i][j]
            && e[5] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[3] == 0
            && e[4] == 0
            && e[0] == 0
            && e[2] == res[i][j]
            && e[6] == res[i][j]
            && e[1] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[2] == 0
            && e[6] == 0
            && e[1] == 0
            && e[3] == res[i][j]
            && e[4] == res[i][j]
            && e[0] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[0] == 0
            && e[5] == 0
            && e[1] == 0
            && e[3] == res[i][j]
            && e[7] == res[i][j]
            && e[2] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[7] == 0
            && e[3] == 0
            && e[4] == 0
            && e[2] == res[i][j]
            && e[6] == res[i][j]
            && e[1] == res[i][j]
            && e[5] == res[i][j]
            && e[0] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[4] == 0
            && e[0] == 0
            && e[5] == 0
            && e[1] == res[i][j]
            && e[6] == res[i][j]
            && e[2] == res[i][j]
            && e[7] == res[i][j]
            && e[3] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[5] == 0
            && e[1] == 0
            && e[6] == 0
            && e[2] == res[i][j]
            && e[7] == res[i][j]
            && e[3] == res[i][j]
            && e[4] == res[i][j]
            && e[0] == res[i][j]
        {
            res[i][j] = 0;
        } else if e[6] == 0
            && e[2] == 0
            && e[7] == 0
            && e[3] == res[i][j]
            && e[4] == res[i][j]
            && e[0] == res[i][j]
            && e[5] == res[i][j]
            && e[1] == res[i][j]
        {
            res[i][j] = 0;
        }
    }
    for _ in 0..3 {
        for i in 0..h {
            for j in 0..w {
                power_play(i, j, &mut res);
            }
        }
        for i in (0..h).rev() {
            for j in 0..w {
                power_play(i, j, &mut res);
            }
        }
        for i in 0..h {
            for j in (0..w).rev() {
                power_play(i, j, &mut res);
            }
        }
        for i in (0..h).rev() {
            for j in (0..w).rev() {
                power_play(i, j, &mut res);
            }
        }
    }

    let res = {
        let mut buf = vec![vec![0; N]; N];
        buf.iter_mut()
            .zip(res)
            .for_each(|(to, from)| to[..from.len()].copy_from_slice(&from));
        buf
    };

    if cfg!(feature = "tayu_local") {
        eprintln!("time: {}", tm.elapsed().unwrap().as_millis());
        eprintln!("try_count: {}", try_count);
    }

    for res in res {
        println!("{}", res.iter().join(" "))
    }
}
