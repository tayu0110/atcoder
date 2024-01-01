use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut a = (1..=n).collect::<Vec<_>>();
    let mut rev = false;
    let get_index = |idx: usize, rev: bool| {
        if !rev {
            idx
        } else {
            n - 1 - idx
        }
    };
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: usize, y: usize}
            a[get_index(x - 1, rev)] = y;
        } else if ty == 2 {
            rev = !rev;
        } else {
            input! {x: usize}
            println!("{}", a[get_index(x - 1, rev)])
        }
    }
}
