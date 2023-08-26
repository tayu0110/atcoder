use proconio::input;

const MAX: usize = 100000;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut t = vec![0usize; MAX];
    a.into_iter().for_each(|a| t[a] += 1);

    let mut res = 0;
    for i in 1..MAX {
        let j = MAX - i;
        if i < j {
            res += t[i] * t[j];
        } else if i == j {
            res += t[i] * t[i].saturating_sub(1) / 2;
        }
    }

    println!("{}", res)
}
