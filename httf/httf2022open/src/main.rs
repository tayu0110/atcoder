use std::{io::*, collections::VecDeque};

static N: i32 = 20;
static DX: [i32; 4] = [0, 1, 0, -1];
static DY: [i32; 4] = [1, 0, -1, 0];


fn check(x: &i32, y: &i32) -> bool {
    *x >= 0 && *y >= 0 && *x < N && *y < N
}

fn check_wall(x: &i32, y: &i32, dx: &i32, dy: &i32, h: &Vec<Vec<i32>>, v: &Vec<Vec<i32>>) -> bool {
    match (*dx, *dy) {
        (0, 1) => {
            if v[(*y-1) as usize][*x as usize] == 1 {
                return false;
            }
        },
        (0, -1) => {
            if v[*y as usize][*x as usize] == 1 {
                return false;
            }
        },
        (1, 0) => {
            if h[*y as usize][(*x-1) as usize] == 1 {
                return false;
            }
        },
        (_, _) => {
            if h[*y as usize][*x as usize] == 1 {
                return false;
            }
        }
    };
    true
}

fn check_end(ck: &Vec<Vec<bool>>) -> bool {
    for i in 0..N {
        for j in 0..N {
            if !ck[i as usize][j as usize] {
                return false;
            }
        }
    }
    true
}

fn dir_change(now_dir: &char, new_dir: &char) -> String {
    match *now_dir {
        'U' => {
            match *new_dir {
                'U' => "".to_string(),
                'D' => "LL".to_string(),
                'L' => "L".to_string(),
                _ => "R".to_string()
            }
        },
        'D' => {
            match *new_dir {
                'U' => "LL".to_string(),
                'D' => "".to_string(),
                'L' => "R".to_string(),
                _ => "L".to_string()
            }
        },
        'L' => {
            match *new_dir {
                'U' => "R".to_string(),
                'D' => "L".to_string(),
                'L' => "".to_string(),
                _ => "LL".to_string()
            }
        },
        _ => {
            match *new_dir {
                'U' => "L".to_string(),
                'D' => "R".to_string(),
                'L' => "LL".to_string(),
                _ => "".to_string()
            }
        }
    }
}

fn rev_dir(d: &char) -> char {
    match *d {
        'U' => 'D',
        'D' => 'U',
        'L' => 'R',
        _ => 'L'
    }
}

fn short_cut(x: &i32, y: &i32, d: &mut char, h: &Vec<Vec<i32>>, v: &Vec<Vec<i32>>, ck: &mut Vec<Vec<bool>>, res: &mut String) {
    let mut nt = VecDeque::new();
    nt.push_back((*x, *y, 0));
    let mut nck = vec![vec![-1; N as usize]; N as usize];
    let (mut nx, mut ny) = {
        let mut res_x = -1;
        let mut res_y = -1;
        while !nt.is_empty() {
            let (x, y, d) = nt.pop_back().unwrap();
            if nck[y as usize][x as usize] >= 0 {
                continue;
            }
            nck[y as usize][x as usize] = d;
            if !ck[y as usize][x as usize] {
                res_x = x;
                res_y = y;
                break;
            }
            for i in 0..4 {
                let nx = x + DX[i];
                let ny = y + DY[i];
                if !check(&nx, &ny) {
                    continue;
                }
                if !check_wall(&nx, &ny, &DX[i], &DY[i], &h, &v) {
                    continue;
                }
                if nck[ny as usize][nx as usize] >= 0 {
                    continue;
                }
                nt.push_back((nx, ny, d+1));
            }
            if res_x >= 0 && res_y >= 0 {
                break;
            }
        }
        (res_x, res_y)
    };
    if nx < 0 || ny < 0 {
        return;
    }
    let start_x = nx;
    let start_y = ny;
    let mut route = vec![];
    loop {
        if nck[ny as usize][nx as usize] == 0 {
            break;
        }
        route.push((nx, ny));
        for i in 0..4 {
            let tx = nx + DX[i];
            let ty = ny + DY[i];
            if !check(&tx, &ty) {
                continue;
            }
            if nck[ny as usize][nx as usize] - nck[ty as usize][tx as usize] == 1 {
                nx = tx;
                ny = ty;
                break;
            }
        }
    }
    while !route.is_empty() {
        let (nx, ny) = route.pop().unwrap();
        let nd;
        if nx - *x == -1 {
            nd = 'L';
        } else if nx - *x == 1 {
            nd = 'R';
        } else if ny - *y == -1 {
            nd = 'U';
        } else {
            nd = 'D';
        }
        res.push_str(dir_change(&d, &nd).as_str());
        res.push('F');
        *d = nd;
    }
    dfs(&start_x, &start_y, d, h, v, ck, res);
}

fn dfs(x: &i32, y: &i32, d: &mut char, h: &Vec<Vec<i32>>, v: &Vec<Vec<i32>>, ck: &mut Vec<Vec<bool>>, res: &mut String) {
    if ck[*y as usize][*x as usize] {
        return;
    }
    ck[*y as usize][*x as usize] = true;
    let mut now_dir = *d;
    let mut new_dir;
    let mut flag = false;
    for i in 0..4 {
        let ny = y + DY[i];
        let nx = x + DX[i];
        if !check(&nx, &ny) {
            continue;
        }
        if !check_wall(&nx, &ny, &DX[i], &DY[i], &h, &v) {
            continue;
        }
        if ck[ny as usize][nx as usize] {
            continue;
        }
        match (DX[i], DY[i]) {
            (0, 1) => new_dir = 'D',
            (0, -1) => new_dir = 'U',
            (1, 0) => new_dir = 'R',
            (_, _) => new_dir = 'L',
        };
        let mut addition = String::new();
        if now_dir != new_dir {
            addition.push_str(dir_change(&now_dir, &new_dir).as_str());
        }
        addition.push('F');
        res.push_str(addition.as_str());
        dfs(&nx, &ny, &mut new_dir, &h, &v, ck, res);
        if check_end(&ck) {
            return;
        }
        res.push('F');
        now_dir = new_dir;
        flag = true;
    }
    if !flag {
        short_cut(x, y, d, h, v, ck, res);
    }
    res.push_str(dir_change(&now_dir, &rev_dir(&d)).as_str());
    *d = rev_dir(&d);
}

fn rlc(s: &[char]) -> Vec<(char, usize)> {
    let mut i = 0;
    let mut ctr = vec![];
    let mut cur = (s[0], 0);
    loop {
        while i < s.len() && s[i] == cur.0 {
            cur.1 += 1;
            i += 1;
        }
        ctr.push(cur);
        if i == s.len() {
            break;
        } else {
            cur = (s[i], 0);
        }
    }
    ctr
}

fn main() {
    let (si, sj) = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s = s.trim_end().to_owned();
        let mut ss = s.split_whitespace();
        let si: i32 = ss.next().unwrap().parse().unwrap();
        let sj: i32 = ss.next().unwrap().parse().unwrap();
        (si, sj)
    };
    let mut h = vec![];
    for _ in 0..N {
        let hh = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            let mut hh = vec![];
            for hhh in s.trim_end().chars() {
                hh.push(hhh.to_digit(10).unwrap() as i32);
            }
            hh
        };
        h.push(hh);
    }
    let mut v = vec![];
    for _ in 0..N-1 {
        let vv = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            let mut vv = vec![];
            for vvv in s.trim_end().chars() {
                vv.push(vvv.to_digit(10).unwrap() as i32);
            }
            vv
        };
        v.push(vv);
    }
    let mut res = String::new();
    let mut ck = vec![vec![false; N as usize]; N as usize];
    let mut dir = 'U';
    dfs(&sj, &si, &mut dir, &h, &v, &mut ck, &mut res);
    let res_with_rls = rlc(&res.chars().collect::<Vec<char>>().as_slice());
    let mut ans = String::new();
    for (c, sz) in res_with_rls {
        if sz > 1 {
            ans.push_str(sz.to_string().as_str());
        }
        ans.push_str(c.to_string().as_str());
    }
    if ans.len() > 10000 {
        ans = ans.chars().take(10000).collect();
    }
    println!("{}", ans);
}
