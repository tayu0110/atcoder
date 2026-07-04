use cpio::*;

const P: usize = 17;

fn main() {
    scan!(n: usize, mut p: [u32; n-1], q: usize, query: [(u32, u32); q]);
    p.iter_mut().for_each(|p| *p -= 1);
    p.insert(0, u32::MAX);

    let mut rank = vec![u32::MAX; n];
    rank[0] = 0;
    let mut stack = vec![];
    for i in 1..n {
        if rank[i] == u32::MAX {
            stack.push(i);
            while let Some(&now) = stack.last() {
                if now == 0 {
                    continue;
                }

                let par = p[now] as usize;
                if rank[par] == u32::MAX {
                    stack.push(par);
                } else {
                    stack.pop();
                    rank[now] = rank[par] + 1;
                }
            }
        }
    }

    let mut doubling = vec![u32::MAX; n * (P + 1)];
    doubling[..n].copy_from_slice(&p);
    let doubling = doubling.chunks_exact_mut(n).collect::<Box<_>>();
    for i in 0..P {
        for j in 0..n {
            if doubling[i][j] != u32::MAX {
                doubling[i + 1][j] = doubling[i][doubling[i][j] as usize];
            }
        }
    }

    for (x, mut y) in query.into_iter().map(|(x, y)| (x - 1, y - 1)) {
        let mut diff = rank[y as usize] - rank[x as usize];
        if diff <= P as u32 {
            for _ in 0..diff - 1 {
                y = doubling[0][y as usize];
            }
        } else {
            for i in (0..P + 1).rev() {
                if diff > 1 << i {
                    diff -= 1 << i;
                    y = doubling[i][y as usize];
                }
            }
        }

        putln!(y + 1);
    }
}
