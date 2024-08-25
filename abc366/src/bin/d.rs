use proconio::*;

fn main() {
    input! {n: usize, a: [[[i64; n]; n]; n], q: usize}

    let mut b = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                b[i + 1][j + 1][k + 1] = a[i][j][k];
            }
        }
    }

    for i in 0..=n {
        for j in 0..=n {
            for k in 0..n {
                b[i][j][k + 1] += b[i][j][k];
            }
        }
    }
    for i in 0..=n {
        for k in 0..=n {
            for j in 0..n {
                b[i][j + 1][k] += b[i][j][k];
            }
        }
    }
    for j in 0..=n {
        for k in 0..=n {
            for i in 0..n {
                b[i + 1][j][k] += b[i][j][k];
            }
        }
    }

    for _ in 0..q {
        input! {lx: usize, rx: usize, ly: usize, ry: usize, lz: usize, rz: usize}
        let (lx, ly, lz) = (lx - 1, ly - 1, lz - 1);

        println!(
            "{}",
            b[rx][ry][rz] - b[rx][ry][lz] - b[rx][ly][rz] - b[lx][ry][rz]
                + b[lx][ly][rz]
                + b[lx][ry][lz]
                + b[rx][ly][lz]
                - b[lx][ly][lz]
        );
    }
}
