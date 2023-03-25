use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n], m: usize, b: [usize; m]}
    let b = b.into_iter().collect::<std::collections::HashSet<_>>();

    let mut res = 0;
    for a in a {
        res += a;

        if b.contains(&res) {
            res = 0;
        }
    }

    println!("{}", res);
}
