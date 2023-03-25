// use itertools::Itertools;
use proconio::input;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize}

        let mut res = k;

        if n % 2 == 1 && res > 1 {
            let m = n / 2;
            let l = k / 2;

            if (m - 1) & (l - 1) == (l - 1) {
                if k % 2 == 0 {
                    res += 1;
                } else {
                    res -= 1;
                }
            }
        }

        let m = n / 2;
        let l = (res + 1) / 2;

        if (m - 1) & (l - 1) == (l - 1) {
            if res % 2 == 1 {
                res += 1;
            } else {
                res -= 1;
            }
        }

        println!("{}", res);
    }
}
