use proconio::input;

fn main() {
    input! {n: usize, q: usize}

    let mut v = vec![];
    for _ in 0..n {
        input! {l: usize, a: [usize; l]}

        v.push(a);
    }

    for _ in 0..q {
        input! {s: usize, t: usize}

        println!("{}", v[s - 1][t - 1]);
    }
}
