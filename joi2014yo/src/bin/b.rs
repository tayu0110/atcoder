use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut cnt = vec![0; n + 1];
    for b in b {
        for (i, &a) in a.iter().enumerate() {
            if a <= b {
                cnt[i + 1] += 1;
                break;
            }
        }
    }

    let &max = cnt.iter().max().unwrap();
    println!("{}", cnt.into_iter().position(|p| p == max).unwrap())
}
