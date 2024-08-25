use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut res = -1;
    for i in 0..n {
        for j in i + 1..n {
            let t = a[i] * a[j];
            let s = t.to_string().chars().collect::<Vec<_>>();
            if s.windows(2)
                .all(|v| v[0] < v[1] && (v[0] as usize).abs_diff(v[1] as usize) == 1)
            {
                res = res.max(t);
            }
        }
    }

    println!("{res}")
}
