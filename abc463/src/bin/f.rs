use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n * 2]}

    let &max = a.iter().max().unwrap();

    let mut ret = vec![usize::MAX; n * 2];
    let f = a.chunks_exact(2).any(|a| a[0] == max && a[1] == max);
    if f {
        for i in 0..2 * n {
            if a[i] != max {
                ret[i] = 0;
            }
        }
    } else {
        for i in 0..2 * n {
            if a[i] + 1 < max {
                ret[i] = 0;
            }
        }
    }
}
