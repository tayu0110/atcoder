use proconio::input;

fn main() {
    input! {h: [usize; 3], w: [usize; 3]}

    let mut res = 0usize;
    for i in 1..=h[0] {
        for j in 1..=h[0] - i {
            let k = h[0] - i - j;
            if k == 0 {
                continue;
            }
            for a in 1..=h[1] {
                if i + a >= w[0] {
                    continue;
                }
                for b in 1..=h[1] - a {
                    if j + b >= w[1] {
                        continue;
                    }
                    let c = h[1] - a - b;
                    if c == 0 {
                        continue;
                    }
                    if k + c >= w[2] {
                        continue;
                    }

                    let (p, q, r) = (w[0] - i - a, w[1] - j - b, w[2] - k - c);
                    if p + q + r == h[2] {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}
