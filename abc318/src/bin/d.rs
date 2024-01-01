use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! {d: usize}
            t[i][j] = d;
        }
    }

    let mut dp = vec![0; 1 << n];
    for i in 0..1 << n {
        for j in 0..n {
            if i & (1 << j) != 0 {
                continue;
            }

            for k in j + 1..n {
                if i & (1 << k) != 0 {
                    continue;
                }
                let next = i | (1 << j) | (1 << k);
                dp[next] = dp[next].max(dp[i] + t[j][k]);
            }
        }
    }

    println!("{}", dp.iter().max().unwrap())
}
// use itertools::Itertools;
// use proconio::*;

// fn solve(p: Vec<usize>, q: Vec<usize>, t: &Vec<Vec<usize>>) -> usize {
//     let n = p.len();
//     let mut res = 0;
//     for v in (0..q.len()).permutations(n) {
//         let mut sum = 0;
//         for (i, j) in v.into_iter().enumerate() {
//             let (p, q) = (p[i], q[j]);
//             sum += t[p.min(q)][p.max(q)];
//         }
//         res = res.max(sum);
//     }

//     res
// }

// fn main() {
//     input! {n: usize}

//     let mut t = vec![vec![0; n]; n];
//     for i in 0..n {
//         for j in i + 1..n {
//             input! {d: usize}
//             t[i][j] = d;
//         }
//     }

//     let mask = (1 << n) - 1;
//     let mut checked = vec![false; 1 << n];
//     let mut res = 0;
//     for i in 0usize..1 << n {
//         if i.count_ones() as usize != n / 2 {
//             continue;
//         }

//         if checked[mask ^ i] {
//             continue;
//         }

//         let mut p = vec![];
//         let mut q = vec![];
//         for j in 0..n {
//             if i & (1 << j) != 0 {
//                 p.push(j);
//             } else {
//                 q.push(j);
//             }
//         }

//         res = res.max(solve(p, q, &t));
//         checked[i] = true;
//     }

//     println!("{res}")
// }
