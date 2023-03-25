#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize}

    let mut res = vec![vec![0; n]; n];

    if n % 2 == 0 {
        let max = n * n;
        let mut set = std::collections::HashSet::new();
        for i in 1..=max {
            set.insert(i);
        }

        for i in 0..n {
            res[n / 2 - 1][i] = i * 2 + 1;
            set.remove(&(i * 2 + 1));
        }

        for i in 1..=n {
            res[n / 2][i-1] = n * n - i * 2;
            set.remove(&(n * n - i * 2));
        }

        let mut now = 1;
        for i in 0..n/2-1 {
            for j in 0..n {
                while !set.contains(&now) {
                    now += 2;
                }

                res[i][j] = now;
                set.remove(&now);
            }
        }

        let mut now = 2;
        for i in n/2+1..n {
            for j in 0..n {
                while !set.contains(&now) {
                    now += 2;
                }

                res[i][j] = now;
                set.remove(&now);
            }
        }
    } else {
        let max = n * n;
        let mut set = std::collections::HashSet::new();
        for i in 1..=max {
            set.insert(i);
        }

        let mut s = 0;
        let mut t = 0;
        for l in (2..=max).step_by(2) {
            for i in (1..=max).step_by(2) {
                let k = l + i;
                if k % max == 0 {
                    continue;
                }
    
                let mut good = false;
                for j in (2..=k).take_while(|j| *j * *j <= k) {
                    if k % j == 0 {
                        good = true;
                        break;
                    }
                }
    
                if good {
                    s = l;
                    t = i;
                    break;
                }
            }
        }

        res[n/2][n/2] = t;
        res[n/2][n/2+1] = s;
        res[n/2+1][n/2] = max - t;
        res[n/2-1][n/2+1] = max - s;
        set.remove(&t);
        set.remove(&s);
        set.remove(&(max-t));
        set.remove(&(max-s));

        for i in 0..n/2 {
            let (mut l, mut r) = (i*2+1, n*n-(i*2+1));
            while !set.contains(&l) {
                l += 2;
                r -= 2;
            }
            res[n / 2][i] = l;
            res[n / 2+1][i] = r;
            set.remove(&l);
            set.remove(&r);
        }
        for i in n/2+2..n {
            let (mut l, mut r) = (i*2+1, n*n-(i*2+1));
            while !set.contains(&l) {
                l += 2;
                r -= 2;
            }
            res[n/2-1][i] = l;
            res[n/2][i] = r;
            set.remove(&l);
            set.remove(&r);
        }

        let mut now = 1;
        for i in 0..n/2 {
            for j in 0..n {
                if i == n/2-1 && j >= n/2+1 {
                    break;
                }
                while !set.contains(&now) {
                    now += 2;
                }

                res[i][j] = now;
                set.remove(&now);
            }
        }

        let mut now = 2;
        for i in n/2+1..n {
            for j in 0..n {
                if i == n/2+1 && j < n/2+1 {
                    continue;
                }
                while !set.contains(&now) {
                    now += 2;
                }

                res[i][j] = now;
                set.remove(&now);
            }
        }
    }

    eprintln!("{}", n);
    for v in res {
        println!("{}", v.iter().join(" "));
    }
}
