use proconio::*;

fn main() {
    input! {n: i64, q: usize}

    let mut res = vec![];
    for i in (1..=n).rev() {
        res.push((i, 0));
    }

    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {c: char}

            let &(nx, ny) = res.last().unwrap();
            match c {
                'R' => res.push((nx + 1, ny)),
                'L' => res.push((nx - 1, ny)),
                'U' => res.push((nx, ny + 1)),
                'D' => res.push((nx, ny - 1)),
                _ => unreachable!(),
            }
        } else {
            input! {p: usize}

            let (x, y) = res[res.len() - p];
            println!("{x} {y}")
        }
    }
}
