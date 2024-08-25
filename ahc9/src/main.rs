use std::{io, cmp};

struct Map {
    h: Vec<Vec<char>>,
    v: Vec<Vec<char>>
}
impl Map {
    fn new() -> Self {
        let mut h = vec![];
        for _ in 0..20 {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            h.push(buf.chars().collect());
        }
        let mut v = vec![];
        for _ in 0..19 {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            v.push(buf.chars().collect());
        }
        Map { h, v }
    }
    fn check(&self, ny: i32, nx: i32, py: i32, px: i32) -> bool {
        if !(0..20).contains(&ny) || !(0..20).contains(&nx) {
            return false;
        }
        if py != ny && self.v[cmp::min(py, ny) as usize][px as usize] == '1' {
            return false;
        }
        if px != nx && self.h[py as usize][cmp::min(px, nx) as usize] == '1' {
            return false;
        }
        true
    }
}

struct Solver {
    sx: usize,
    sy: usize,
    tx: usize,
    ty: usize,
    _p: f32,
    map: Map,
    reached: [[bool; 20]; 20]
}
impl Solver {
    fn new() -> Self {
        let (sy, sx, ty, tx, p) = {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            let mut ws = s.split_ascii_whitespace().to_owned();
            let sy: usize = ws.next().unwrap().parse().unwrap();
            let sx: usize = ws.next().unwrap().parse().unwrap();
            let ty: usize = ws.next().unwrap().parse().unwrap();
            let tx: usize = ws.next().unwrap().parse().unwrap();
            let p: f32 = ws.next().unwrap().parse().unwrap();
            (sy, sx, ty, tx, p)
        };
        let map = Map::new();
        let reached = [[false; 20]; 20];
        Solver { sx, sy, tx, ty, _p: p, map, reached }
    }
    fn dfs(&mut self, y: usize, x: usize, res: &mut String) -> bool {
        if y == self.ty && x == self.tx {
            return true;
        }
        if self.reached[y][x] {
            return false;
        }
        self.reached[y][x] = true;
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];
            if !self.map.check(ny, nx, y as i32, x as i32) {
                continue;
            }
            if self.reached[ny as usize][nx as usize] {
                continue;
            }
            if self.dfs(ny as usize, nx as usize, res) {
                match i {
                    0 => res.push('R'),
                    1 => res.push('D'),
                    2 => res.push('L'),
                    _ => res.push('U')
                }
                return true;
            }
        }
        false
    }
    fn solve(&mut self) {
        let mut res_rev = String::new();
        self.dfs(self.sy, self.sx, &mut res_rev);
        let mut res = String::new();
        while !res_rev.is_empty() {
            res.push(res_rev.pop().unwrap());
        }
        println!("{}", res);
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
