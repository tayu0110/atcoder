use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {n: usize, k: usize, s: Chars};

    let mut t = vec![vec![]; 26];
    for (i, c) in s.iter().enumerate() {
        let code = *c as u8 - 'a' as u8;
        t[code as usize].push(i);
    }
    
    let mut ok = 0;
    let mut res = String::new();
    loop {
        let mut not_found = true;
        let rem = k - res.len();
        for (i, v) in t.iter().enumerate() {
            if v.is_empty() {
                continue;
            }
            let mut l = -1;
            let mut r = v.len() as i32;
            while r - l > 1 {
                let m = (r + l) / 2;
                if v[m as usize] >= ok {
                    r = m;
                } else {
                    l = m;
                }
            }
            if r == v.len() as i32 {
                continue;
            }
            if n - v[r as usize] >= rem {
                res.push(('a' as u8 + i as u8) as char);
                ok = v[r as usize] + 1;
                not_found = false;
                break;
            }
        }

        if not_found || rem == 1 {
            break;
        }
    }

    if res.len() < k {
        for i in ok..n {
            res.push(s[i]);
        }
    }

    println!("{}", res.to_string());
}