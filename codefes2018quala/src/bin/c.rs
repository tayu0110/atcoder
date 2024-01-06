use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut t = 0;
    for &a in &a {
        let mut a = a;
        while a > 0 {
            a >>= 1;
            t += 1;
        }
    }

    let max = std::cmp::min(t, k);
    let mut memo = vec![0; max + 1];
    memo[0] = 1;
    for mut a in a {
        let mut t = vec![a];
        while a > 0 {
            a >>= 1;
            t.push(a);
        }
        let _t = t.len();
    }

    println!("{}", memo[max]);
}
