use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut a: [i64; n]}
        let positive = a.iter().filter(|&&a| a > 0).count();
        let negative = n - positive;
        if positive.abs_diff(negative) > 1 && positive != 0 && negative != 0 {
            println!("No");
            continue;
        }

        if positive == 0 || negative == 0 {
            a.sort_unstable_by_key(|&a| a.abs());
        } else {
            let mut pos = a.iter().copied().filter(|&a| a > 0).collect::<Vec<_>>();
            let mut neg = a.iter().copied().filter(|&a| a < 0).collect::<Vec<_>>();
            pos.sort_unstable_by_key(|&a| a.abs());
            neg.sort_unstable_by_key(|&a| a.abs());
            if pos.len() < neg.len() {
                (pos, neg) = (neg, pos);
            } else if pos.len() == neg.len() && pos[0].abs() > neg[0].abs() {
                (pos, neg) = (neg, pos);
            }
            a.clear();
            for i in 0..neg.len() {
                a.push(pos[i]);
                a.push(neg[i]);
            }
            if pos.len() > neg.len() {
                a.push(*pos.last().unwrap());
            }
        }
        let s = a[0];
        let t = a[1];

        if a[1..].windows(2).all(|v| {
            let (k, l) = (v[0], v[1]);
            t * k == s * l
        }) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
