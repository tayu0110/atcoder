use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    a.iter_mut().for_each(|a| *a -= 1);
    let mut pos = vec![0; n];
    for (i, &a) in a.iter().enumerate() {
        pos[a] = i;
    }

    let mut res = vec![];
    for i in 0..n {
        if a[i] != i {
            let p = pos[i];
            let old = a[i];
            res.push((i + 1, p + 1));
            a.swap(i, p);
            pos.swap(i, old);
        }
    }

    println!("{}", res.len());
    for (i, j) in res {
        println!("{i} {j}")
    }
}
