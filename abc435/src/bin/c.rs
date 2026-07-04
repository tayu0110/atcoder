use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ret = 0;
    let mut cum = vec![0i32; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        if cum[i] > 0 || i == 0 {
            ret += 1;
            let to = (i + a).min(n);
            cum[i] += 1;
            cum[to] -= 1;
        }

        cum[i + 1] += cum[i];
    }

    println!("{ret}")
}
