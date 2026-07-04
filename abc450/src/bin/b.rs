use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = vec![];
    for i in 0..n - 1 {
        input! {c: [usize; n - i - 1]};
        t.push(c);
    }

    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if t[a][c - a - 1] > t[a][b - a - 1] + t[b][c - b - 1] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No")
}
