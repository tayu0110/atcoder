use flow::Dinic;
use proconio::*;

fn main() {
    input! {n: usize, mut c: [marker::Bytes; n]}

    for i in 0..n {
        for j in 0..n {
            if c[i][j] == b'?' {
                continue;
            }
            if (i + j) % 2 == 1 {
                c[i][j] ^= b'B' ^ b'W';
            }
        }
    }

    let mut ff = Dinic::<usize>::new(n * n + 2);
    for i in 0..n {
        for j in 0..n {
            for (di, dj) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < n && nj < n {
                    ff.set_edge(i * n + j, ni * n + nj, 1);
                }
            }
            if c[i][j] == b'B' {
                ff.set_edge(n * n, i * n + j, usize::MAX);
            } else if c[i][j] == b'W' {
                ff.set_edge(i * n + j, n * n + 1, usize::MAX);
            }
        }
    }

    println!("{}", 2 * n * (n - 1) - ff.flow(n * n, n * n + 1))
}
