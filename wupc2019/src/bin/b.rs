use proconio::*;

// 1->2->4->8->6->2
// 3->6->2->4->8->6
// 5->0
// 7->4->8->6->2->4
// 9->8->6->2->4->8

fn f(a: u8) -> u8 {
    match a {
        9 => 3,
        8 => 2,
        6..=7 => 1,
        _ => 0,
    }
}

fn main() {
    input! {h: usize, w: usize, a: [[u8; w]; h]}

    let &max = a.iter().flatten().max().unwrap();
    let exists = a.iter().flatten().any(|&a| a == 5);
    if max == 0 {
        println!("Yes 0");
        return;
    } else if max == 5 {
        println!("Yes 1");
        return;
    } else if !exists {
        println!("No");
        return;
    }

    if h.min(w) == 1 {
        let a = a.into_iter().flatten().collect::<Vec<_>>();
        let n = a.len();
        let mut res = u8::MAX;
        for i in 0..n {
            if a[i] != 5 {
                continue;
            }

            let &ma = a[..i].iter().max().unwrap_or(&0);
            let &mb = a[i + 1..].iter().max().unwrap_or(&0);

            res = res.min(f(ma) + f(mb) + 1);
        }

        println!("Yes {res}");
    } else {
        println!("Yes {}", f(max) + 1);
    }
}
