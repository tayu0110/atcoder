use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [[usize; 6]; n]}

    let mut t = vec![vec![vec![std::u32::MAX; 105]; 105]; 105];
    for (i, v) in p.into_iter().enumerate() {
        for x in v[0] + 1..=v[3] {
            for y in v[1] + 1..=v[4] {
                for z in v[2] + 1..=v[5] {
                    t[x][y][z] = i as u32;
                }
            }
        }
    }

    let mut buf = vec![];
    for x in 1..=101 {
        for y in 1..=101 {
            for z in 1..=101 {
                if t[x][y][z] == std::u32::MAX {
                    continue;
                }
                for dx in [!0, 1] {
                    for dy in [!0, 1] {
                        for dz in [!0, 1] {
                            for (dx, dy, dz) in [(dx, 0, 0), (0, dy, 0), (0, 0, dz)] {
                                let (nx, ny, nz) =
                                    (x.wrapping_add(dx), y.wrapping_add(dy), z.wrapping_add(dz));

                                let (p, q) = (t[x][y][z], t[nx][ny][nz]);
                                if q == std::u32::MAX || p == q {
                                    continue;
                                }

                                buf.push((p.min(q), p.max(q)));
                            }
                        }
                    }
                }
            }
        }
    }

    buf.sort();
    buf.dedup();

    let mut res = vec![0; n];
    for (a, b) in buf {
        res[a as usize] += 1;
        res[b as usize] += 1;
    }

    println!("{}", res.iter().join("\n"))
}
