use proconio::*;

fn main() {
    input! {_: usize, _: usize, s: marker::Bytes, t: marker::Bytes, q: usize}

    for _ in 0..q {
        input! {w: marker::Bytes}

        let sb = w.iter().copied().all(|b| s.contains(&b));
        let tb = w.iter().copied().all(|b| t.contains(&b));

        if (sb && tb) || (!sb && !tb) {
            println!("Unknown")
        } else if sb {
            println!("Takahashi")
        } else {
            println!("Aoki")
        }
    }
}
