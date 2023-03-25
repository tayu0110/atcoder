use proconio::{marker::Chars, *};

fn main() {
    input! {_: usize, s: Chars}

    let mut set = std::collections::HashSet::new();
    let (mut x, mut y) = (0, 0);
    set.insert((x, y));

    for c in s {
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => unreachable!(),
        }

        if set.contains(&(x, y)) {
            println!("Yes");
            return;
        }

        set.insert((x, y));
    }

    println!("No")
}
