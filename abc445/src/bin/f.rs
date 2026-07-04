use proconio::*;

const M: usize = 32;

fn main() {
    input! {n: usize, k: usize, c: [[usize; n]; n]}

    let mut doubling = vec![vec![vec![usize::MAX; n]; n]; M];
    doubling[0] = c;
    for l in 0..M - 1 {
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    doubling[l + 1][i][j] =
                        doubling[l + 1][i][j].min(doubling[l][i][k] + doubling[l][k][j]);
                }
            }
        }
    }

    for i in 0..n {
        let mut rem = k;
        let mut init = false;
        let mut cost = vec![0; n];
        for j in (0..M).rev() {
            if rem >= (1 << j) {
                rem -= 1 << j;
                if !init {
                    init = true;

                    for k in 0..n {
                        cost[k] = doubling[j][i][k];
                    }
                    continue;
                }

                let mut next = vec![usize::MAX; n];
                for to in 0..n {
                    for from in 0..n {
                        next[to] = next[to].min(cost[from] + doubling[j][from][to]);
                    }
                }
                cost = next;
            }
        }

        assert_eq!(rem, 0);
        println!("{}", cost[i]);
    }
}
