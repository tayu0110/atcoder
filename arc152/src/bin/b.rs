#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, l: usize, mut a: [usize; n]}

    if n == 1 && l - a[0] < a[0] {
        a[0] = l - a[0];
    }

    a.insert(0, 0);
    a.push(l);

    let mut ra = vec![0; n+2];
    for i in (0..n+1).rev() {
        ra[i] = ra[i+1] + (a[i+1] - a[i]);
    }

    a.pop();
    a.remove(0);
    
    ra.pop();
    ra.remove(0);

    // eprintln!("a: {:?}", a);
    // eprintln!("ra: {:?}", ra);

    let mut min = std::usize::MAX;
    for i in 0..n {
        let (mut l, mut r) = (0, n);

        while r - l > 1 {
            let m = (r + l) / 2;
            if ra[m] < a[i] {
                r = m;
            } else {
                l = m;
            }
        }

        if r < n && ra[r] < a[i] {
            // if a[i] - ra[r] < min {
            //     eprintln!("i: {}, r: {}, a[i]-ra[r]: {}", i, r, a[i] - ra[r]);
            // }
            min = std::cmp::min(min, a[i] - ra[r]);
        }
        if ra[l] >= a[i] {
            // if ra[l] - a[i] < min {
            //     eprintln!("i: {}, l: {}, ra[l]-a[i]: {}", i, l, ra[l] - a[i]);
            // }
            min = std::cmp::min(min, ra[l] - a[i]);
        } else {
            min = std::cmp::min(min, a[i] - ra[l]);
        }
    }

    println!("{}", (l+min) * 2);
}
