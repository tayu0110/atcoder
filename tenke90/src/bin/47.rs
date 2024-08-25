use proconio::input;
use proconio::marker::Chars;

fn modpow(a: i64, n: i64, p: i64) -> i64 {
    if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else {
        let mut res = modpow(a, n/2, p);
        res = res * res % p;
        if n % 2 == 1 {
            res = res * a % p;
        }
        res
    }
}

fn solve(s: &[char], t: &[char], c: [char; 3]) -> usize {
    let t = t.iter().map(|&nc| if nc == c[0] { nc } else { (nc as u8 ^ c[1] as u8 ^ c[2] as u8) as char }).collect::<Vec<_>>();
    let n = s.len() as i64;

    let p = 998244353i64;
    let b = 111222333i64;
    let mut pow = vec![];
    let mut hs = vec![0];
    let mut ht = vec![0];
    for i in 0..n {
        pow.push(modpow(b, i, p));
        hs.push((hs.last().unwrap() * b + (s[i as usize] as u8) as i64) % p);
        ht.push((ht.last().unwrap() * b + (t[i as usize] as u8) as i64) % p);
    }
    pow.push(modpow(b, n, p));

    let mut res = 0;
    for i in 1-n..0 {
        let mut sh = (hs[n as usize] - pow[(n+i) as usize] * hs[(-i) as usize]) % p;
        let mut th = (ht[(n+i) as usize] - pow[(n+i) as usize] * ht[0]) % p;
        sh = (sh + p) % p;
        th = (th + p) % p;
        // eprintln!("i: {}, sh: {}, th: {}", i, sh, th);
        if sh == th {
            res += 1;
        }
    }
    for i in 0..n {
        let mut sh = (hs[(n-i) as usize] - pow[(n-i) as usize] * hs[0]) % p;
        let mut th = (ht[n as usize] - pow[(n-i) as usize] * ht[i as usize]) % p;
        sh = (sh + p) % p;
        th = (th + p) % p;
        // eprintln!("i: {}, sh: {}, th: {}", i, sh, th);
        if sh == th {
            res += 1;
        }
    }

    res
}

fn main() {
    input! {_n: i64, s: Chars, t: Chars};

    let mut res = solve(&s, &t, ['R', 'G', 'B']);
    res += solve(&s, &t, ['G', 'R', 'B']);
    res += solve(&s, &t, ['B', 'R', 'G']);

    println!("{}", res);
}