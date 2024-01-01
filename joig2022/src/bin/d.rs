use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {_: usize, _: usize, n: usize, p: [(i64, i64); n]}

    let mut map = HashMap::new();
    for (a, b) in p {
        *map.entry((a, b)).or_insert(0) += 1;
        for (dx, dy) in vec![
            (0, -1),
            (-1, 0),
            (-1, -1),
            (-2, 0),
            (0, -2),
            (-2, -2),
            (-1, -2),
            (-2, -1),
        ] {
            let (na, nb) = (a + dx, b + dy);
            map.entry((na, nb)).or_insert(0);
        }
    }

    let mut res = 0;
    for (a, b) in map.keys() {
        let mut t = 0;
        for (dx, dy) in vec![
            (0, 1),
            (0, 0),
            (1, 0),
            (1, 1),
            (2, 0),
            (0, 2),
            (2, 2),
            (1, 2),
            (2, 1),
        ] {
            let (na, nb) = (a + dx, b + dy);
            t += map.get(&(na, nb)).unwrap_or(&0);
        }

        res = res.max(t);
    }

    println!("{res}")
}
