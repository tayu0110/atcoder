use proconio::{marker::Chars, *};
// use rand::{thread_rng, Rng};

fn naive(k: usize, s: &[char]) -> usize {
    let len = s.len();
    let mut res = 0;

    for i in 0u32..1 << len {
        if i.count_ones() != k as u32 {
            continue;
        }

        let mut s = s.to_vec();
        for j in 0..len {
            if i & (1 << j) != 0 {
                if s[j] == 'X' {
                    s[j] = 'Y';
                } else {
                    s[j] = 'X';
                }
            }
        }

        let mut sum = 0;
        for v in s.windows(2) {
            if v[0] == 'Y' && v[1] == 'Y' {
                sum += 1;
            }
        }

        res = std::cmp::max(sum, res);
    }

    res
}

fn main() {
    input! {n: usize, mut k: usize, mut s: Chars}
    // for _ in 0..50 {
    // let mut rng = thread_rng();
    // let n: usize = rng.gen_range(1, 20);
    // let mut k: usize = rng.gen_range(0, n + 1);
    // let mut s = vec![];
    // for _ in 0..n {
    //     s.push(rng.gen_range(b'X', b'Z') as char);
    // }

    let naive_res = if cfg!(debug_assertions) {
        eprintln!("n: {}, k: {}, s: {:?}", n, k, s);
        naive(k, &s)
    } else {
        0
    };

    let mut res = 0;
    for v in s.windows(2) {
        if v[0] == 'Y' && v[1] == 'Y' {
            res += 1;
        }
    }

    let mut rlc = vec![];
    for &c in &s {
        if rlc.is_empty() {
            rlc.push((1, c, true));
            continue;
        }

        match rlc.last_mut() {
            Some((cnt, nc, _)) if *nc == c => *cnt += 1,
            _ => rlc.push((1, c, false)),
        }
    }

    if rlc.len() == 1 {
        if rlc[0].1 == 'Y' {
            assert_eq!(res, n - 1);
            if k == n {
                println!("{}", 0);
            } else {
                println!("{}", res - k);
            }
        } else {
            assert_eq!(res, 0);
            println!("{}", k.saturating_sub(1));
        }
        return;
        // continue;
    }

    rlc.last_mut().unwrap().2 = true;

    let mut rem = vec![];
    let mut t = vec![];
    for &(cnt, c, f) in &rlc {
        if c == 'X' {
            if f {
                rem.push(cnt);
            } else {
                t.push(cnt);
            }
        }
    }

    t.sort();
    for cnt in t {
        if k >= cnt {
            res += cnt + 1;
            k -= cnt;
        } else {
            res += k;
            k = 0;
            break;
        }
    }

    rem.sort();
    for cnt in rem {
        if k >= cnt {
            res += cnt;
            k -= cnt;
        } else {
            res += k;
            k = 0;
        }
    }

    if k == 0 {
        println!("{}", res);
        return;
        // continue;
    }

    assert_eq!(res, n - 1);

    let mut rem = vec![];
    let mut t = vec![];
    for &(cnt, c, f) in rlc.iter() {
        if c == 'Y' {
            if f {
                rem.push(cnt);
            } else {
                t.push(cnt);
            }
        }
    }

    rem.sort();
    rem.reverse();
    for cnt in rem {
        if k >= cnt {
            res -= cnt;
            k -= cnt;
        } else {
            res -= k;
            k = 0;
        }
    }

    if k > 0 {
        t.sort();
        t.reverse();
        for cnt in t {
            if k >= cnt {
                res -= cnt + 1;
                k -= cnt;
            } else if k > 0 {
                res -= k + 1;
                k = 0;
                break;
            }
        }
    }

    assert_eq!(k, 0);

    debug_assert_eq!(res, naive_res);

    println!("{}", res);
    // }
}
