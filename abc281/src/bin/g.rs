use proconio::input;

fn main() {
    input! {n: usize, m: usize}

    // for n in 3..7 {
    //     let mut res = 0;
    //     'base: for j in 0..(1 << (n * n)) {
    //         let mut dist = vec![vec![0; n]; n];
    //         for k in 0..n * n {
    //             if j & (1 << k) != 0 {
    //                 dist[k / n][k % n] = 1;
    //                 if k / n == k % n {
    //                     continue 'base;
    //                 }
    //             } else {
    //             }
    //         }

    //         for k in 0..n {
    //             dist[k][k] = 0;
    //         }

    //         for k in 0..n {
    //             for i in 0..n {
    //                 for j in 0..n {
    //                     dist[i][j] = std::cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
    //                 }
    //             }
    //         }

    //         for k in 0..n - 1 {
    //             if dist[0][k] >= dist[0][n - 1] {
    //                 continue 'base;
    //             }
    //         }

    //         res += 1;
    //     }

    //     println!("n: {}, res: {}", n, res);
    // }
}
