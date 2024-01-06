// TODO
// use matrix::Matrix;
// use proconio::*;
// use static_modint::{Mod1000000007, StaticModint};

// type Modint = StaticModint<Mod1000000007>;

// fn sub(n: usize, start: usize, g: &Vec<Vec<u32>>) -> Vec<Modint> {
//     let mut dp = vec![vec![Modint::zero(); 1 << n]; n];
//     dp[start][1 << start] = Modint::one();
//     for i in (1 << start)..(1 << n) {
//         for now in 0..n {
//             if i & (1 << now) == 0 {
//                 continue;
//             }
//             let tmp = dp[now][i];
//             for j in 0..n {
//                 if i & (1 << j) != 0 {
//                     continue;
//                 }
//                 if g[now][j] == 0 {
//                     continue;
//                 }

//                 dp[j][i | (1 << j)] += tmp;
//             }
//         }
//     }

//     let mut res = vec![Modint::zero(); n];
//     for i in 0..n {
//         for j in (1 << start)..(1 << n) {
//             res[i] += dp[i][j];
//         }
//     }
//     res
// }

fn main() {
    // input! {h: usize, r: usize, g: [[u32; r]; r]}

    // let mut mat = Matrix::zeros(r, r);
    // for i in 0..r {
    //     let t = sub(r, i, &g);
    //     for j in 0..r {
    //         mat.set(i, j, t[j]);
    //     }
    // }
    // let mat = mat.pow(h);
    // let mut f = Matrix::zeros(r, 1);
    // f.set(0, 0, Modint::one());
    // let res = mat.mul(&f);
    // println!("{}", res.get(0, 0))
}
