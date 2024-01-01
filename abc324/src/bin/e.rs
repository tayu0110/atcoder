use proconio::*;

fn main() {
    input! {n: usize, t: marker::Chars, s: [marker::Chars; n]}

    let mut f = vec![];
    let mut b = vec![];
    for s in s {
        // fromt
        {
            let (mut si, mut ti) = (0, 0);
            while si < s.len() && ti < t.len() {
                if s[si] == t[ti] {
                    si += 1;
                    ti += 1;
                } else {
                    si += 1;
                }
            }

            f.push(ti);
        }
        // back
        {
            let (mut si, mut ti) = (s.len(), t.len());
            while 0 < si && 0 < ti {
                if s[si - 1] == t[ti - 1] {
                    si -= 1;
                    ti -= 1;
                } else {
                    si -= 1;
                }
            }

            b.push(t.len() - ti);
        }
    }

    b.sort();
    b.reverse();

    let mut res = 0;
    for f in f {
        let (mut l, mut r) = (-1, b.len() as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if f + b[m as usize] >= t.len() {
                l = m;
            } else {
                r = m;
            }
        }

        res += r as usize;
    }

    println!("{res}")

    // let mut c = vec![vec![]; 26];
    // for (i, &t) in t.iter().enumerate() {
    //     let idx = t as usize - b'a' as usize;
    //     c[idx].push(i);
    // }

    // let mut f = vec![];
    // let mut b = vec![];
    // for s in s {
    //     // forward
    //     {
    //         let mut now = -1 as i32;
    //         let mut bad = false;
    //         for &s in &s {
    //             let idx = s as usize - b'a' as usize;
    //             let c = &c[idx];
    //             let (mut l, mut r) = (-1, c.len() as i32);
    //             while r - l > 1 {
    //                 let m = (r + l) / 2;
    //                 if now < c[m as usize] as i32 {
    //                     r = m;
    //                 } else {
    //                     l = m;
    //                 }
    //             }

    //             if r == c.len() as i32 {
    //                 bad = true;
    //                 break;
    //             }
    //             now = c[r as usize] as i32;
    //         }
    //         if !bad {
    //             b.push(now as usize);
    //         }
    //     }
    //     // backward
    //     {
    //         let mut now = n;
    //         let mut bad = false;
    //         for &s in s.iter().rev() {
    //             let idx = s as usize - b'a' as usize;
    //             let c = &c[idx];
    //             let (mut l, mut r) = (-1, c.len() as i32);
    //             while r - l > 1 {
    //                 let m = (r + l) / 2;
    //                 if c[m as usize] < now {
    //                     l = m;
    //                 } else {
    //                     r = m;
    //                 }
    //             }

    //             if l < 0 {
    //                 bad = true;
    //                 break;
    //             }
    //             now = c[l as usize];
    //         }

    //         if !bad {
    //             f.push(now);
    //         }
    //     }
    // }

    // f.sort();
    // b.sort();

    // let mut res = 0i64;
    // for f in f {
    //     let (mut l, mut r) = (-1, b.len() as i32);
    //     while r - l > 1 {
    //         let m = (r + l) / 2;
    //         if b[m as usize] < f {
    //             l = m;
    //         } else {
    //             r = m;
    //         }
    //     }

    //     res += (l + 1) as i64;
    // }

    // println!("{res}")
}
