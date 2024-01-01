use proconio::*;
use utility::CumSum2D;

fn main() {
    input! {n: usize, q: usize, p: [marker::Bytes; n]}

    let cum = p
        .into_iter()
        .map(|p| {
            p.into_iter()
                .map(|c| (c == b'B') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let cum = CumSum2D::new(cum);

    for _ in 0..q {
        input! {a: usize, b: usize, c: usize, d: usize}

        println!("{}", cum.sum_as_infinite_grid(a..c + 1, b..d + 1))
    }

    // let mut cum = vec![vec![0; n + 1]; n + 1];
    // for i in 0..n {
    //     for j in 0..n {
    //         cum[i + 1][j + 1] = (p[i][j] == b'B') as usize;
    //     }
    // }

    // for i in 0..=n {
    //     for j in 0..n {
    //         cum[i][j + 1] += cum[i][j];
    //     }
    // }
    // for j in 0..=n {
    //     for i in 0..n {
    //         cum[i + 1][j] += cum[i][j];
    //     }
    // }

    // let f = |u: usize, l: usize, d: usize, r: usize| -> usize {
    //     cum[d + 1][r + 1] + cum[u][l] - cum[d + 1][l] - cum[u][r + 1]
    // };
    // let g = |a: usize, b: usize| -> usize {
    //     let mut res = cum[n][n] * ((a + 1) / n) * ((b + 1) / n);
    //     if (a + 1) % n != 0 {
    //         res += f(0, 0, a % n, n - 1) * ((b + 1) / n);
    //     }
    //     if (b + 1) % n != 0 {
    //         res += f(0, 0, n - 1, b % n) * ((a + 1) / n);
    //     }
    //     if (a + 1) % n != 0 && (b + 1) % n != 0 {
    //         res += f(0, 0, a % n, b % n);
    //     }

    //     res
    // };

    // for _ in 0..q {
    //     input! {a: usize, b: usize, c: usize, d: usize}

    //     let mut res = g(c, d);

    //     if a > 0 && b > 0 {
    //         res += g(a - 1, b - 1);
    //     }
    //     if a > 0 && d > 0 {
    //         res -= g(a - 1, d);
    //     }
    //     if b > 0 && c > 0 {
    //         res -= g(c, b - 1);
    //     }

    //     println!("{res}")
    // }
}
