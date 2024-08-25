use proconio::*;

fn main() {
    input! {n: usize, l: usize, x: [usize; n], t: [usize; 3]}

    let mut memo = vec![usize::MAX; l + 4];
    memo[0] = 0;
    for i in 0..l {
        if x.binary_search(&(i + 1)).is_ok() {
            memo[i + 1] = memo[i + 1].min(memo[i] + t[0] + t[2]);
        } else {
            memo[i + 1] = memo[i + 1].min(memo[i] + t[0]);
        }

        if x.binary_search(&(i + 2)).is_ok() {
            memo[i + 2] = memo[i + 2].min(memo[i] + t[0] + t[1] + t[2]);
        } else if i + 1 == l {
            memo[i + 2] = memo[i + 2].min(memo[i] + t[0] / 2 + t[1] / 2);
        } else {
            memo[i + 2] = memo[i + 2].min(memo[i] + t[0] + t[1]);
        }

        if x.binary_search(&(i + 4)).is_ok() {
            memo[i + 4] = memo[i + 4].min(memo[i] + t[0] + t[1] * 3 + t[2]);
        } else if i + 1 == l {
            memo[i + 4] = memo[i + 4].min(memo[i] + t[0] / 2 + t[1] / 2);
        } else if i + 2 == l {
            memo[i + 4] = memo[i + 4].min(memo[i] + t[0] / 2 + t[1] / 2 * 3);
        } else if i + 3 == l {
            memo[i + 4] = memo[i + 4].min(memo[i] + t[0] / 2 + t[1] / 2 * 5);
        } else {
            memo[i + 4] = memo[i + 4].min(memo[i] + t[0] + t[1] * 3);
        }
    }

    println!("{}", memo[l..].iter().min().unwrap())
}
