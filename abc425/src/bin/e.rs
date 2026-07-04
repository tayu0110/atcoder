use proconio::*;

const N: usize = 5000;

fn main() {
    input! {t: usize, m: usize}

    let mut com = vec![vec![0; N + 1]; N + 1];
    com[0][0] = 1;
    for i in 1..=N {
        com[i][0] = 1;
        for j in 1..=i {
            com[i][j] = com[i - 1][j] + com[i - 1][j - 1];
            com[i][j] %= m;
        }
    }

    for _ in 0..t {
        input! {n: usize, c: [usize; n]}

        let mut sum = c.iter().sum::<usize>();
        let mut res = 1;
        for c in c {
            res *= com[sum][c];
            res %= m;
            sum -= c;
        }

        println!("{}", res);
    }
}
