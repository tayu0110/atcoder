use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut t = vec![];
    for _ in 0..m {
        input! {k: usize}
        let mut buf = vec![];
        for _ in 0..k {
            input! {a: u16, b: u16}
            buf.push((a, b));
        }
        t.push(buf);
    }

    for i in 0u16..1 << n {
        let mut f = true;
        for b in &t {
            f &= b.iter().any(|&(a, b)| (i >> (a - 1)) & 1 == b);
        }

        if f {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
