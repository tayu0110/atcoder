use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = Vec::with_capacity(n);
    for _ in 0..n {
        input! {a: usize, b: usize}

        if a > b {
            println!("No");
            return;
        }

        t.push((b, b - a, a));
    }

    t.sort();

    let mut now = 0;
    for (_, deadline, a) in t {
        if now <= deadline {
            now += a;
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
