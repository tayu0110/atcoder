use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut t = 0;
    for mut h in h {
        let c = h / 5;
        let k = 3 * c;
        h -= c * 5;

        let v = if t % 3 == 0 {
            [1, 1, 3]
        } else if t % 3 == 1 {
            [1, 3, 1]
        } else {
            [3, 1, 1]
        };
        t += k;

        if h == 0 {
            continue;
        }

        for v in v {
            t += 1;
            if h <= v {
                break;
            }
            h -= v;
        }
    }

    println!("{t}")
}
