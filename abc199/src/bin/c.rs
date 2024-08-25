use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Chars, q: usize}

    let mut f = false;
    for _ in 0..q {
        input! {t: usize, a: usize, b: usize}

        if t == 1 {
            if !f {
                s.swap(a - 1, b - 1);
            } else {
                s.swap((a - 1 + n) % (2 * n), (b - 1 + n) % (2 * n));
            }
        } else {
            f = !f;
        }
    }

    if f {
        let (a, b) = s.split_at(n);
        s = b
            .iter()
            .cloned()
            .chain(a.iter().cloned())
            .collect::<Vec<_>>();
    }

    println!("{}", s.iter().collect::<String>())
}
