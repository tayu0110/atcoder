use im_rc::HashSet;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, x: usize, w: [usize; n], k: [usize; q]};

    let sum = w.iter().fold(0usize, |sum, x| sum + *x);
    let mut nsum = sum;
    let mut t = vec![0; n];
    for (i, nw) in w.iter().enumerate() {
        t[i] += *nw;
        if i < n-1 {
            t[i+1] += t[i];
        }
    }

    // eprintln!("{:?}", t);

    let mut b = vec![];
    for i in 0..n {
        let mut nx = x;
        let mut num = 0;
        let mut target = i;
        if nsum < nx {
            nx -= nsum;
            num += n - i;
            target = 0;
        }
        if sum < nx {
            num += n * (nx / sum);
            nx %= sum;
        }
        // eprintln!("nsum: {}, target: {}", nsum, target);

        let mut l = target as i32 - 1;
        let mut r = n as i32;
        while r - l > 1 {
            let m = (r + l) / 2;
            let mut sum = t[m as usize];
            if target > 0 {
                sum -= t[target-1];
            }
            if sum < nx {
                l = m;
            } else {
                r = m;
            }
        }
        // eprintln!("l: {}, r: {}", l, r);
        num += r as usize + 1;
        if target > 0 {
            num -= target;
        }
        b.push(((r as usize + 1) % n, num));
        nsum -= w[i];
    }

    // eprintln!("{:?}", b);

    let mut now = 0;
    let mut ck = HashSet::new();
    let mut nt = vec![];
    while !ck.contains(&now) {
        ck.insert(now);
        nt.push((now, b[now]));

        now = b[now].0;
    }

    // eprintln!("now: {}", now);
    // eprintln!("nt: {:?}", nt);

    let g = {
        let mut idx = 0;
        for (j, &(i, _)) in nt.iter().enumerate() {
            if i == now {
                idx = j;
                break;
            }
        }
        idx
    };

    // eprintln!("g: {}", g);

    for nk in k {
        let nk = nk-1;
        if nk <= g {
            let (_, ans) = nt[nk];
            println!("{}", ans.1);
            continue;
        }

        let nnk = (nk - g) % (nt.len() - g);
        let (_, ans) = nt[g + nnk];
        println!("{}", ans.1);
    }
}
