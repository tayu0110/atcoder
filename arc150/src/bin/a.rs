#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut k: usize, mut s: Chars}

        let (mut pl, mut pr) = (std::usize::MAX, std::usize::MAX);
        for i in 0..n {
            if s[i] == '1' {
                pl = i;
                break;
            }
        }
        for i in (0..n).rev() {
            if s[i] == '1' {
                pr = i;
                break;
            }
        }

        let mut cnt = 0;
        let (mut l, mut r) = (0, 0);
        while l < n {
            if s[l] == '0' {
                l += 1;
                continue;
            }
            if l > 0 && s[l-1] == '1' {
                l += 1;
                continue;
            }

            if r < l {
                r = l;
            }

            while r - l < k && r < n && s[r] != '0' {
                r += 1;
            }

            if r-l == k && ((r == n) || (r < n && s[r] != '1')) {
                if pl == std::usize::MAX {
                    cnt += 1;
                } else if l <= pl && pr <= r {
                    cnt += 1;
                }
            }

            l += 1;
        }

        // eprintln!("cnt: {}", cnt);
        if cnt == 1 {
            println!("Yes");
        } else {
            println!("No");
        }

        // let (mut l, mut r) = (std::usize::MAX, std::usize::MAX);
        // for i in 0..n {
        //     if s[i] == '1' {
        //         l = i;
        //         break;
        //     }
        // }
        // for i in (0..n).rev() {
        //     if s[i] == '1' {
        //         r = i;
        //         break;
        //     }
        // }

        // if l == std::usize::MAX {
        //     let mut max = 0;
        //     let mut cnt = 0;
        //     for i in 0..n {
        //         if s[i] != '?' {
        //             max = std::cmp::max(max, cnt);
        //             cnt = 0;
        //         } else {
        //             cnt += 1;
        //         }
        //     }

        //     max = std::cmp::max(max, cnt);

        //     if max == k {
        //         println!("Yes");
        //     } else {
        //         println!("No");
        //     }
        //     continue;
        // }

        // let mut bad = false;
        // for i in l..=r {
        //     if s[i] == '?' || s[i] == '1' {
        //         if k == 0 {
        //             bad = true;
        //             break;
        //         }
        //         s[i] = '1';
        //         k -= 1;
        //     } else if s[i] == '0' {
        //         bad = true;
        //         break;
        //     }
        // }

        // if bad {
        //     println!("No");
        //     continue;
        // }

        // if k == 0 {
        //     println!("Yes");
        // } else {
        //     if l > 0 && s[l-1] == '?' && r+1 < n && s[r+1] == '?' {
        //         let (mut nl, mut nr) = (l, r);
        //         for i in r+1..n {
        //             if s[i] == '0' {
        //                 break;
        //             }

        //             k -= 1;
        //             nr = i;
        //             if k == 0 {
        //                 break;
        //             }
        //         }
        //         for i in (0..l).rev() {
        //             if s[i] == '0' {
        //                 break;
        //             }

        //             if k > 0 {
        //                 k -= 1;
        //                 nl = i;
        //             }
        //             if k == 0 {
        //                 break;
        //             }
        //         }

        //         if k > 0 {
        //             println!("No");
        //         } else if (nl > 0 && s[nl-1] == '?') || (nr+1 < n && s[nr+1] == '?') {
        //             println!("No");
        //         } else {
        //             println!("Yes");
        //         }
        //         continue;
        //     } else if l > 0 && s[l-1] == '?' {
        //         for i in (0..l).rev() {
        //             if s[i] == '?' {
        //                 k -= 1;
        //                 if k == 0 {
        //                     break;
        //                 }
        //             } else if s[i] == '0' {
        //                 bad = true;
        //                 break;
        //             }
        //         }
        //     } else if r+1 < n && s[r+1] == '?' {
        //         for i in r..n {
        //             if s[i] == '?' {
        //                 k -= 1;
        //                 if k == 0 {
        //                     break;
        //                 }
        //             } else if s[i] == '0' {
        //                 bad = true;
        //                 break;
        //             }
        //         }
        //     } else {
        //         println!("No");
        //         continue;
        //     }

        //     if bad {
        //         println!("No");
        //     } else {
        //         println!("Yes");
        //     }
        // }
    }
}
