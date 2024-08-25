use proconio::*;
use rustc_hash::{FxHashMap, FxHashSet};
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;
type HashSet<V> = FxHashSet<V>;
type HashMap<K, V> = FxHashMap<K, V>;

fn main() {
    input! {n: usize, k: usize, s: marker::Bytes}

    if n == 1 {
        println!("1");
        return;
    }

    if n == 2 {
        if s[0] == s[1] {
            println!("1")
        } else {
            println!("2");
        }
        return;
    }

    if n == 3 {
        let mut set = HashSet::default();
        set.insert(s.to_vec());
        set.insert([s[0], s[2], s[1]].into_iter().collect());
        set.insert([s[1], s[0], s[2]].into_iter().collect());
        set.insert([s[2], s[1], s[0]].into_iter().collect());

        if k >= 2 {
            set.insert([s[1], s[2], s[0]].into_iter().collect());
            set.insert([s[2], s[0], s[1]].into_iter().collect());
        }

        println!("{}", set.len());
        return;
    }

    fn naive_one(s: &[u8]) -> usize {
        let mut set = HashSet::default();
        set.insert(s.to_vec());
        for _ in 0..2 {
            let mut new = HashSet::default();
            for s in set {
                for i in 0..s.len() {
                    for j in i + 1..s.len() {
                        if s[i] != s[j] {
                            let mut t = s.to_vec();
                            t.swap(i, j);
                            new.insert(t);
                        }
                    }
                }
            }
            set = new;
        }

        set.len()
    }
    eprintln!("naive: {}", naive_one(&s));

    let mut dp = HashMap::default();
    // (nk, lack_a, lack_b, lack_c, over_a, over_b, over_c)
    dp.insert((0, 0, 0, 0, 0, 0, 0), Modint::one());
    for s in s {
        let mut new = dp.clone();
        for ((nk, la, lb, lc, oa, ob, oc), v) in dp {
            if s == b'A' {
                if la > 0 {
                    // backward
                    if ob > 0 {
                        *new.entry((nk, la - 1, lb, lc, oa, ob - 1, oc))
                            .or_insert(Modint::zero()) += v;
                    }
                    if oc > 0 {
                        *new.entry((nk, la - 1, lb, lc, oa, ob, oc - 1))
                            .or_insert(Modint::zero()) += v;
                    }
                }
                // forward
                if nk < k {
                    *new.entry((nk + 1, la, lb + 1, lc, oa + 1, ob, oc))
                        .or_insert(Modint::zero()) += v;
                    *new.entry((nk + 1, la, lb, lc + 1, oa + 1, ob, oc))
                        .or_insert(Modint::zero()) += v;
                }
            }
            if s == b'B' {
                if lb > 0 {
                    if oa > 0 {
                        *new.entry((nk, la, lb - 1, lc, oa - 1, ob, oc))
                            .or_insert(Modint::zero()) += v;
                    }
                    if oc > 0 {
                        *new.entry((nk, la, lb - 1, lc, oa, ob, oc - 1))
                            .or_insert(Modint::zero()) += v;
                    }
                }
                if nk < k {
                    *new.entry((nk + 1, la + 1, lb, lc, oa, ob + 1, oc))
                        .or_insert(Modint::zero()) += v;
                    *new.entry((nk + 1, la, lb, lc + 1, oa, ob + 1, oc))
                        .or_insert(Modint::zero()) += v;
                }
            }
            if s == b'C' {
                if lc > 0 {
                    if oa > 0 {
                        *new.entry((nk, la, lb, lc - 1, oa - 1, ob, oc))
                            .or_insert(Modint::zero()) += v;
                    }
                    if ob > 0 {
                        *new.entry((nk, la, lb, lc - 1, oa, ob - 1, oc))
                            .or_insert(Modint::zero()) += v;
                    }
                }
                if nk < k {
                    *new.entry((nk + 1, la + 1, lb, lc, oa, ob, oc + 1))
                        .or_insert(Modint::zero()) += v;
                    *new.entry((nk + 1, la, lb + 1, lc, oa, ob, oc + 1))
                        .or_insert(Modint::zero()) += v;
                }
            }
        }

        dp = new;
    }

    let mut res = Modint::zero();
    for i in 0..=k {
        let t = *dp.get(&(i, 0, 0, 0, 0, 0, 0)).unwrap_or(&Modint::zero());
        eprintln!("{t}");
        res += t;
    }

    // let mut dp = vec![];
    // for k in 0..=k {
    //     dp.push(vec![vec![]; k + 1]);
    //     for a in 0..=k {
    //         for b in (0..=k).take_while(|b| a + b <= k) {
    //             let c = k - a - b;
    //             dp[k][a].push(vec![Modint::zero(); c + 1]);
    //         }
    //     }
    // }
    // dp[0][0][0][0] = Modint::one();

    // for s in s {
    //     let mut new = dp.clone();

    //     for nk in 0..dp.len() {
    //         let ma = dp[nk].len();
    //         for a in 0..ma {
    //             let mb = dp[nk][a].len();
    //             for b in 0..mb {
    //                 let mc = dp[nk][a][b].len();
    //                 for c in 0..mc {
    //                     if s == b'A' && a > 0 {
    //                         new[nk][a - 1][b][c] += dp[nk][a][b][c];
    //                     }
    //                     if s == b'B' && b > 0 {
    //                         new[nk][a][b - 1][c] += dp[nk][a][b][c];
    //                     }
    //                     if s == b'C' && c > 0 {
    //                         new[nk][a][b][c - 1] += dp[nk][a][b][c];
    //                     }

    //                     if nk < k {
    //                         if s != b'A' {
    //                             new[nk + 1][a + 1][b][c] += dp[nk][a][b][c];
    //                         }
    //                         if s != b'B' {
    //                             new[nk + 1][a][b + 1][c] += dp[nk][a][b][c];
    //                         }
    //                         if s != b'C' {
    //                             new[nk + 1][a][b][c + 1] += dp[nk][a][b][c];
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     dp = new;
    // }

    // let mut res = Modint::zero();
    // for i in 0..=k {
    //     res += dp[i][0][0][0];
    // }

    println!("{res}");
}
