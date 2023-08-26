use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n-1], b: [usize; n-2]};

    let mut res = vec![std::usize::MAX; n + 1];
    res[1] = 0;
    for i in 0..n - 1 {
        if i < n - 1 {
            res[i + 2] = res[i + 2].min(res[i + 1] + a[i]);
        }
        if i < n - 2 {
            res[i + 3] = res[i + 3].min(res[i + 1] + b[i]);
        }
    }

    println!("{}", res[n])
}
