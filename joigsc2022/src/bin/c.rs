use proconio::*;

fn main() {
    input! {n: usize, p: [(u32, u32, u32); n]}

    let mut x = p
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| (x, y, z, i as u32))
        .collect::<Vec<_>>();
    let mut y = x.clone();
    let mut z = y.clone();
    x.sort_unstable_by_key(|v| v.0);
    y.sort_unstable_by_key(|v| v.1);
    z.sort_unstable_by_key(|v| v.2);
    let xyz = [x, y, z];

    let mut bad = vec![false; n];
    let mut t = [n; 3];

    'main: while t[0] > 0 && t[1] > 0 && t[2] > 0 {
        for i in 0..3 {
            let a = xyz[i][t[i] - 1];
            if bad[a.3 as usize] {
                t[i] -= 1;
                continue 'main;
            }
            for j in 0..3 {
                if i == j {
                    continue;
                }
                let b = xyz[j][t[j] - 1];
                if a.3 == b.3 {
                    bad[a.3 as usize] = true;
                    t[i] -= 1;
                    t[j] -= 1;
                    continue 'main;
                }

                let f = match (i, j) {
                    (_, 0) => a.0 == b.0,
                    (_, 1) => a.1 == b.1,
                    (_, 2) => a.2 == b.2,
                    _ => false,
                };
                if f {
                    bad[a.3 as usize] = true;
                    t[i] -= 1;
                    continue 'main;
                }
            }
        }

        break;
    }

    if t[0] > 0 && t[1] > 0 && t[2] > 0 {
        let x = xyz[0][t[0] - 1];
        let y = xyz[1][t[1] - 1];
        let z = xyz[2][t[2] - 1];
        println!("{}", x.0 + y.1 + z.2);
    } else {
        println!("-1")
    }
}
