use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {n: usize}

    let mut set = HashSet::new();
    for _ in 0..n {
        input! {a: usize, b: usize}
        set.insert((a.min(b), a.max(b)));
    }

    println!("{}", set.len())
}
