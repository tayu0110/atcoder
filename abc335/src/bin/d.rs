use proconio::*;

fn update(dx: usize, dy: usize) -> (usize, usize) {
    match (dx, dy) {
        (1, 0) => (0, 1),
        (0, 1) => (!0, 0),
        (usize::MAX, 0) => (0, !0),
        (0, usize::MAX) => (1, 0),
        _ => unreachable!(),
    }
}

fn main() {
    input! {n: usize}

    let mut res = vec![vec![0; n]; n];
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (1, 0);
    let mut now = 1;
    while now <= n * n - 1 {
        res[y][x] = now;
        now += 1;

        let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
        if nx >= n || ny >= n || res[ny][nx] != 0 {
            (dx, dy) = update(dx, dy);
        }

        (x, y) = (x.wrapping_add(dx), y.wrapping_add(dy));
    }

    for res in res {
        print!("{}", res[0]);
        for r in res.into_iter().skip(1) {
            if r == 0 {
                print!(" T");
            } else {
                print!(" {r}");
            }
        }
        println!();
    }
}
