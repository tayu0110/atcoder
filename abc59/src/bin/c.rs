use proconio::*;

fn solve(mut def: i64, a: &[i64]) -> i64 {
    let mut now = a[0];
    for i in 1..a.len() {
        let n = now.signum();
        if now + a[i] == 0 {
            def += 1;
            now += (a[i].abs() + 1) * a[i].signum()
        } else if n != (now + a[i]).signum() {
            now += a[i];
        } else {
            now += a[i];
            def += now.abs() + 1;
            now = -now.signum();
        }
    }

    def
}

fn main() {
    input! {n: usize, mut a: [i64; n]}

    println!(
        "{}",
        if a[0] < 0 {
            let res = solve(0, &a);
            let def = a[0].abs() + 1;
            a[0] = 1;
            res.min(solve(def, &a))
        } else if a[0] > 0 {
            let res = solve(0, &a);
            let def = a[0].abs() + 1;
            a[0] = -1;
            res.min(solve(def, &a))
        } else {
            a[0] = 1;
            let res = solve(1, &a);
            a[0] = -1;
            res.min(solve(1, &a))
        }
    )
}
