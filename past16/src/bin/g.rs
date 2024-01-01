use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 3 * n]}
    let m = 3 * n;

    let mut memo = vec![0; 1 << m];
    memo[0] = 1;
    for i in 0..1 << m {
        for j in 0..m {
            if i & (1 << j) != 0 {
                continue;
            }
            for k in j + 1..m {
                for l in k + 1..m {
                    if i & ((1 << j) | (1 << k) | (1 << l)) == 0 {
                        let mut v = [a[j], a[k], a[l]];
                        v.sort_unstable();
                        if v[0] + v[1] > v[2] {
                            memo[i | (1 << j) | (1 << k) | (1 << l)] += memo[i];
                        }
                    }
                }
            }

            break;
        }
    }

    println!("{}", memo[(1 << m) - 1])
}
