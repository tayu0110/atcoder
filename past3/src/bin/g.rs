use proconio::input;

fn main() {
    input! {n: usize, x: i32, y: i32, p: [(i32, i32); n]}

    let set = p.into_iter().collect::<std::collections::HashSet<_>>();
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 0, 0));
    let mut map = std::collections::HashMap::new();
    while let Some((nd, px, py)) = nt.pop_front() {
        if map.contains_key(&(px, py)) {
            continue;
        }
        map.insert((px, py), nd);

        for (dx, dy) in vec![(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)] {
            let (nx, ny) = (px + dx, py + dy);
            if map.contains_key(&(nx, ny)) {
                continue;
            }
            if nx < -210 || nx > 210 || ny < -210 || ny > 210 {
                continue;
            }
            if set.contains(&(nx, ny)) {
                continue;
            }
            nt.push_back((nd + 1, nx, ny));
        }
    }

    if let Some(res) = map.get(&(x, y)) {
        println!("{}", res);
    } else {
        println!("-1");
    }
}
