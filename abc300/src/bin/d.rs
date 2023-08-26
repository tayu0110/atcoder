use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: usize}

    let sq = n.sqrt() + 1;
    let mut prime = vec![];
    let mut t = vec![0; sq + 1];
    for i in 2..=sq {
        if t[i] == 0 {
            prime.push(i);
            for j in (2..).take_while(|&j| i * j <= sq) {
                t[i * j] = i;
            }
        }
    }

    let mut v = vec![vec![]; sq + 1];
    for i in 1..prime.len() {
        for j in 0..i {
            let mul = prime[i] * prime[j];
            if mul * mul <= n {
                v[mul] = vec![j, i];
            } else {
                break;
            }
        }
    }

    let mut res = 0;
    for i in 1..=sq {
        if !v[i].is_empty() {
            let (a, c) = (v[i][0], v[i][1]);
            assert_eq!(prime[a] * prime[c], i);

            let b_max = n / i / i;
            if b_max <= prime[a] {
                continue;
            }

            if b_max < prime[c] {
                let (mut l, mut r) = (0, prime.len());
                while r - l > 1 {
                    let m = (r + l) / 2;
                    if prime[m] <= b_max {
                        l = m;
                    } else {
                        r = m;
                    }
                }
                res += l - a;
            } else {
                res += c - 1 - a;
            }
        }
    }

    println!("{}", res)
}
