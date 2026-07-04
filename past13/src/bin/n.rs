use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], q: usize}

    let mut index = (0..n).collect::<Vec<_>>();
    index.sort_unstable_by_key(|&i| a[i]);

    for _ in 0..q {
        input! {l: usize, r: usize}
        if l == r {
            println!("0");
            continue;
        }

        let (l, r) = (l - 1, r - 1);
        let &start = index.iter().find(|&i| (l..=r).contains(&i)).unwrap();
        let pos = index.iter().position(|&i| i == start).unwrap();
        let mut res = 0;
        let mut now = a[start];
        for &i in index.iter().skip(pos + 1).filter(|&i| (l..=r).contains(&i)) {
            res += (a[i] - now).pow(2);
            now = a[i];
        }
        println!("{res}")
    }
}
