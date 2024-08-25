use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize}

    let mut t = vec![];
    for _ in 0..m {
        input! {c: usize, a: [usize; c], r: char}
        t.push((c, a, r));
    }

    let mut res = 0;
    for i in 0..1 << n {
        let mut bad = false;
        for (_, a, r) in &t {
            let mut sum = 0;
            for &a in a {
                if i & (1 << (a - 1)) != 0 {
                    sum += 1;
                }
            }

            if (*r == 'o' && sum < k) || (*r == 'x' && sum >= k) {
                bad = true;
                break;
            }
        }

        if !bad {
            res += 1;
        }
    }

    println!("{res}")
}
