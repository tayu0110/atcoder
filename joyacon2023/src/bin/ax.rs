use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut res = 0;
    let set = p
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect::<std::collections::HashSet<_>>();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if set.contains(&(i, j)) && set.contains(&(i, k)) && set.contains(&(j, k)) {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
