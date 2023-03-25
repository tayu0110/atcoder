use proconio::input;

fn main() {
    input! {n: usize, colors: usize, d: [[usize; colors]; colors], c: [[usize; n]; n]}

    let mut memo = vec![vec![0; colors]; 3];
    for i in 0..n {
        for j in 0..n {
            memo[(i + j) % 3][c[i][j] - 1] += 1;
        }
    }

    let mut res = std::usize::MAX;
    let calc = |i: usize, index: usize| {
        memo[index]
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .fold(0, |s, (j, &v)| s + v * d[j][i])
    };
    for i in 0..colors {
        let d1 = calc(i, 0);
        for j in 0..colors {
            if i == j {
                continue;
            }

            let d2 = calc(j, 1);
            for k in 0..colors {
                if i == k || j == k {
                    continue;
                }

                let d3 = calc(k, 2);

                res = std::cmp::min(res, d1 + d2 + d3);
            }
        }
    }

    println!("{}", res)
}
