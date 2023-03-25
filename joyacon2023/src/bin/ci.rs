use proconio::input;

fn main() {
    input! {n: usize, x: i32, y: i32, a: [i32; n]}

    let mut set = std::collections::HashSet::new();
    set.insert((a[0], 0));

    for (i, a) in a.into_iter().skip(1).enumerate() {
        let mut new = std::collections::HashSet::new();
        for &(x, y) in &set {
            if i % 2 == 0 {
                if y + a <= 20000 {
                    new.insert((x, y + a));
                }
                if y - a >= -20000 {
                    new.insert((x, y - a));
                }
            } else {
                if x + a <= 20000 {
                    new.insert((x + a, y));
                }
                if x - a >= -20000 {
                    new.insert((x - a, y));
                }
            }
        }

        set = new;
    }

    if set.contains(&(x, y)) {
        println!("Yes")
    } else {
        println!("No")
    }
}
