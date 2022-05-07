use std::io::*;
const MAP_SIZE: usize = 30;
const ADJ: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

enum CreatureType {
    _NoCreature,
    _Cow,
    _Pig,
    _Rabbit,
    _Dog,
    _Cat,
    Human
}

struct Pet {
    num: usize,
    pos: Vec<(usize, usize, usize)>
}

impl Pet {
    fn new(map: &mut Map) -> Self {
        let num = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned().parse().unwrap()
        };
        let mut pos = vec![];
        for _ in 0..num {
            let (mut px, mut py, pt) = {
                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();
                let mut ws = s.split_whitespace().to_owned();
                let px: usize = ws.next().unwrap().parse().unwrap();
                let py: usize = ws.next().unwrap().parse().unwrap();
                let pt: usize = ws.next().unwrap().parse().unwrap();
                (px, py, pt)
            };
            px -= 1;
            py -= 1;
            pos.push((px, py, pt));
            map.add(px, py, pt);
        }
        Self { num, pos }
    }
    fn parse_move_str(mv: String) -> (i32, i32) {
        let mut res = (0, 0);
        for c in mv.chars() {
            res = match c {
                'U' => (res.0-1, res.1),
                'D' => (res.0+1, res.1),
                'L' => (res.0, res.1-1),
                'R' => (res.0, res.1+1),
                _ => res
            };
        }
        res
    }
    fn resolve(&mut self, map: &mut Map) {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let p_mv: Vec<&str> = s.split_whitespace().collect();
        for i in 0..self.num {
            let (px, py, pt) = self.pos[i];
            map.remove(px, py, pt);
            let (dx, dy) = Pet::parse_move_str(p_mv[i].to_string());
            let nx = (px as i32) + dx;
            let ny = (py as i32) + dy;
            map.add(nx as usize, ny as usize, pt);
            self.pos[i] = (nx as usize, ny as usize, pt);
        }
    }
}

struct Human {
    num: usize,
    pos: Vec<(usize, usize)>
}

impl Human {
    fn new(map: &mut Map) -> Self {
        let num = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned().parse().unwrap()
        };
        let mut pos = vec![];
        for _ in 0..num {
            let (mut hx, mut hy) = {
                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();
                let mut ws = s.split_whitespace().to_owned();
                let hx: usize = ws.next().unwrap().parse().unwrap();
                let hy: usize = ws.next().unwrap().parse().unwrap();
                (hx, hy)
            };
            hx -= 1;
            hy -= 1;
            pos.push((hx, hy));
            map.add(hx, hy, CreatureType::Human as usize);
        }
        Self { num, pos }
    }
    fn parse_move_str(dx: i32, dy: i32, is_disable: bool) -> char {
        let mut res = match (dx, dy) {
            (-1, 0) => 'U',
            (1, 0) => 'D',
            (0, -1) => 'L',
            (0, 1) => 'R',
            _ => '.'
        };
        if is_disable {
            res = res.to_ascii_lowercase();
        }
        res
    }
    fn solve(&mut self, map: &mut Map) {
        let mut res = String::new();
        for i in 0..self.num {
            let (hx, hy) = self.pos[i];
            if !map.is_only_human(hx, hy) {
                res.push('.');
                continue;
            }
            let (mut px, mut py) = (0, 0);
            let mut is_disable = false;
            for i in 0..4 {
                let (dx, dy) = ADJ[i];
                let nx = hx as i32 + dx;
                let ny = hy as i32 + dy;
                if map.try_disable(nx, ny) {
                    px = dx;
                    py = dy;
                    is_disable = true;
                    break;
                }
            }
            res.push(Human::parse_move_str(px, py, is_disable));
        }
        println!("{}", res);
    }
}

#[derive(Clone, Copy)]
struct Cell {
    c_set: usize,
    c_num: [i32; 7]
}

impl Cell {
    fn new() -> Self {
        Self {
            c_set: 0,
            c_num: [0; 7]
        }
    }
    fn remove(&mut self, c_type: usize) {
        self.c_num[c_type] -= 1;
        if self.c_num[c_type] == 0 {
            self.c_set ^= 1 << c_type;
        }
    }
    fn add(&mut self, c_type: usize) {
        self.c_num[c_type] += 1;
        self.c_set |= 1 << c_type;
    }
    fn disable(&mut self) -> Option<()> {
        if Cell::is_empty(self) {
            self.c_set = 1;
            Some(())
        } else {
            None
        }
    }
    fn is_empty(&self) -> bool {
        self.c_set == 0
    }
    fn is_only_human(&self) -> bool {
        self.c_set == 1 << (CreatureType::Human as i32)
    }
}

struct Map {
    map: [[Cell; MAP_SIZE]; MAP_SIZE]
}

impl Map {
    fn new() -> Self {
        let map = [[Cell::new(); MAP_SIZE]; MAP_SIZE];
        Self { map }
    }
    fn remove(&mut self, x: usize, y: usize, c_type: usize) {
        self.map[x][y].remove(c_type);
    }
    fn add(&mut self, x: usize, y: usize, c_type: usize) {
        self.map[x][y].add(c_type);
    }
    fn is_out_of_range(x: i32, y: i32) -> bool {
        x < 0 || x >= MAP_SIZE as i32 || y < 0 || y >= MAP_SIZE as i32
    }
    fn try_disable(&mut self, x: i32, y: i32) -> bool {
        if Map::is_out_of_range(x, y) {
            return false;
        }
        if !self.is_empty(x as usize, y as usize) {
            return false;
        }
        for i in 0..4 {
            let (dx, dy) = ADJ[i];
            let nx = x + dx;
            let ny = y + dy;
            if Map::is_out_of_range(nx, ny) {
                continue;
            }
            if self.map[nx as usize][ny as usize].is_only_human() {
                continue;
            }
            if self.map[nx as usize][ny as usize].is_empty() {
                continue;
            }
            return false;
        }
        match self.map[x as usize][y as usize].disable() {
            Some(()) => true,
            None => false
        }
    }
    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.map[x][y].is_empty()
    }
    fn is_only_human(&self, x: usize, y: usize) -> bool {
        self.map[x][y].is_only_human()
    }
}

fn main() {
    let mut map = Map::new();
    let mut pet = Pet::new(&mut map);
    let mut human = Human::new(&mut map);
    const LOOP: usize = 300;
    for _ in 0..LOOP {
        human.solve(&mut map);
        pet.resolve(&mut map);
    }
}
