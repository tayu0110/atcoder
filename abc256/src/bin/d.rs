use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize); n]};

    const MX: usize = 200010;
    let mut t = vec![0; MX+1];
    for (l, r) in p {
        t[l] += 1;
        t[r] -= 1;
    }

    for i in 0..MX {
        t[i+1] += t[i];
    }

    let mut prev = 0;
    for i in 0..MX {
        if t[i] > 0 {
            if t[prev] == 0 {
                prev = i;
            }
            continue;
        } else if t[prev] == 0 {
            prev = i;
        } else {
            println!("{} {}", prev, i);
            prev = i;
        }
    }
}