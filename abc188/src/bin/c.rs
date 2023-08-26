use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 1 << n]}

    let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
    while a.len() > 2 {
        let mut new = vec![];
        for a in a.chunks_exact(2) {
            let (li, la) = a[0];
            let (ri, ra) = a[1];
            if la > ra {
                new.push((li, la));
            } else {
                new.push((ri, ra));
            }
        }
        a = new;
    }

    if a[0].1 > a[1].1 {
        println!("{}", a[1].0 + 1)
    } else {
        println!("{}", a[0].0 + 1)
    }
}
