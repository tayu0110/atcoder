use proconio::*;

const N: usize = 5;

fn main() {
    input! {a: [usize; N]}

    let mut res = vec![];
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                res.push(a[i] + a[j] + a[k]);
            }
        }
    }

    res.sort_unstable();
    println!("{}", res[res.len() - 3])
}
