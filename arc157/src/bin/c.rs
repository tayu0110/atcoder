use modint::{Mod998244353, MontgomeryModint};
use proconio::{marker::Chars, *};

type Modint = MontgomeryModint<Mod998244353>;

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut dp = vec![vec![vec![Modint::zero(); 3]; w]; h];
    dp[0][0][0] = Modint::one();

    for i in 0..h {
        for j in 0..w {
            for (dx, dy) in [(0, 1), (1, 0)] {
                let (x, y) = (j + dx, i + dy);
                if x >= w || y >= h {
                    continue;
                }

                let a = dp[i][j].clone();
                let mut b = dp[y][x].clone();
                let yy = s[i][j] == 'Y' && s[y][x] == 'Y';
                if !yy {
                    b[0] += a[0];
                    b[1] += a[1];
                    b[2] += a[2];
                } else {
                    b[0] += a[0];
                    b[1] += a[0] + a[1];
                    b[2] += a[0] + a[1] * Modint::new(2) + a[2];
                }

                dp[i][j] = a;
                dp[y][x] = b;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1][2]);
}
