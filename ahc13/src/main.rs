use proconio::input;
use proconio::marker::Chars;

struct UnionFind {
    tree: Vec<i32>
}
impl UnionFind {
    fn new(size: usize) -> Self {
        let mut tree = vec![];
        for _ in 0..size {
            tree.push(-1);
        }
        UnionFind { tree }
    }
    #[allow(dead_code)]
    fn root(&self, index: usize) -> usize {
        if self.tree[index] < 0 {
            index
        } else {
            self.root(self.tree[index] as usize)
        }
    }
    #[allow(dead_code)]
    fn size(&self, index: usize) -> usize {
        -self.tree[self.root(index)] as usize
    }
    #[allow(dead_code)]
    fn is_same(&self, left: usize, right: usize) -> bool {
        self.root(left) == self.root(right)
    }
    #[allow(dead_code)]
    fn merge(&mut self, left: usize, right: usize) -> bool {
        let mut rl = self.root(left);
        let mut rr = self.root(right);
        if rl == rr {
            return false;
        }
        if self.tree[rl] > self.tree[rr] {
            std::mem::swap(&mut rl, &mut rr);
        }
        self.tree[rl] += self.tree[rr];
        self.tree[rr] = rl as i32;
        true
    }
}

fn main() {
    input! {n: usize, k: usize, c: [Chars; n]};
    let mut max_move = k * 100;

    // let mut x = vec![];
    let mut y = vec![];

    let mut map = vec![];
    for v in c {
        let buf = v.into_iter().fold(vec![], |mut v, nc| { v.push((nc as u8 - b'0') as i32); v });
        map.push(buf);
    }

    let mut uf = UnionFind::new(n * n);
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    for i in 0..n {
        for j in 0..n {
            if map[i][j] <= 0 {
                continue;
            }
            for k in 0..4 {
                if max_move == 0 {
                    break;
                }
                let (mut nx, mut ny) = (j as i32, i as i32);
                loop {
                    nx += dx[k];
                    ny += dy[k];
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
                        break;
                    }
                    if map[ny as usize][nx as usize] != map[i][j] && map[ny as usize][nx as usize] != 0 {
                        break;
                    }
                    if map[ny as usize][nx as usize] == map[i][j] {
                        if !uf.is_same((ny * n as i32 + nx) as usize, i * n + j) {
                            uf.merge((ny * n as i32 + nx) as usize, i * n + j);
                            y.push((i, j, ny, nx));
                            ny -= dy[k];
                            nx -= dx[k];
                            max_move -= 1;
                            while ny != i as i32 || nx != j as i32 {
                                map[ny as usize][nx as usize] = -map[i][j];
                                ny -= dy[k];
                                nx -= dx[k];
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    println!("0");
    println!("{}", y.len());
    for (e, f, g, h) in y {
        println!("{} {} {} {}", e, f, g, h);
    }
}
