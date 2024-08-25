use proconio::*;

fn main() {
    input! {n: usize, result: [[usize; 4]; n * (n - 1) / 2]}

    let mut p = vec![0; n + 1];
    for v in result {
        if v[2] == v[3] {
            p[v[0]] += 1;
            p[v[1]] += 1;
        } else {
            p[v[(v[2] < v[3]) as usize]] += 3;
        }
    }

    for i in 1..=n {
        println!("{}", p[1..].iter().filter(|&&q| q > p[i]).count() + 1);
    }
}
