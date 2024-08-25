use proconio::*;
use rand::*;

const BALLS: usize = 645;
const MAX: usize = 30;
// 仮に最下段30個以外がすべてErrorだった場合、さらに走査を続けてすべて揃えてしまうよりもここで打ち切ったほうがスコアが高いはず？
//      →　LIMITを絞ってもスコアに変化はないので、より少ない回数で揃えるのが重要
const LIMIT: usize = 14340;

fn swap_in_vec(
    t: &mut [Vec<usize>],
    h1: usize,
    w1: usize,
    h2: usize,
    w2: usize,
    res: &mut Vec<(usize, usize, usize, usize)>,
) {
    let tmp = t[h1][w1];
    t[h1][w1] = t[h2][w2];
    t[h2][w2] = tmp;
    res.push((h1, w1, h2, w2));
}

fn solve(mut t: Vec<Vec<usize>>, res: &mut Vec<(usize, usize, usize, usize)>) -> Vec<Vec<usize>> {
    for i in (1..MAX).rev() {
        for j in 0..i {
            if t[i - 1][j] < t[i][j] && t[i - 1][j] < t[i][j + 1] {
                continue;
            }
            if t[i][j] < t[i][j + 1] {
                swap_in_vec(&mut t, i, j, i - 1, j, res);
            } else {
                swap_in_vec(&mut t, i, j + 1, i - 1, j, res);
            }
            if res.len() == LIMIT {
                return t;
            }
        }
    }
    t
}

fn check(t: &[Vec<usize>]) -> bool {
    for i in 1..MAX {
        for j in 0..i {
            if t[i][j] < t[i - 1][j] || t[i][j + 1] < t[i - 1][j] {
                return false;
            }
        }
    }
    true
}

fn naive(mut t: Vec<Vec<usize>>) -> Vec<(usize, usize, usize, usize)> {
    let mut res = vec![];
    while !check(&t) && res.len() < LIMIT {
        t = solve(t, &mut res);
    }
    res
}

#[allow(unused)]
fn croll(t: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 1..MAX {
        for j in 0..i {
            if t[i][j] < t[i - 1][j] || t[i][j + 1] < t[i - 1][j] {
                res.push((i - 1, j));
            }
        }
    }
    res
}

#[allow(unused)]
fn randomized(mut t: Vec<Vec<usize>>) -> Vec<(usize, usize, usize, usize)> {
    let mut rng = thread_rng();
    let mut invalid = croll(&t);
    let mut res = vec![];
    while !invalid.is_empty() && res.len() < LIMIT {
        let len = invalid.len();
        for _ in 0..len {
            let rem = invalid.len();
            let tar: usize = rng.gen_range(0, rem);
            let (r, c) = invalid.remove(tar);
            if t[r][c] < t[r + 1][c] && t[r][c] < t[r + 1][c + 1] {
                continue;
            }

            if t[r + 1][c] < t[r + 1][c + 1] {
                swap_in_vec(&mut t, r, c, r + 1, c, &mut res);
            } else {
                swap_in_vec(&mut t, r, c, r + 1, c + 1, &mut res);
            }
        }
        invalid = croll(&t);
    }
    res
}

type V4 = Vec<(usize, usize, usize, usize)>;

fn min_first(limit: usize, mut t: Vec<Vec<usize>>) -> (V4, Vec<Vec<usize>>) {
    let mut rng = thread_rng();
    let mut res = vec![];
    for i in 0..BALLS {
        let (mut r, mut c) = (0, 0);
        'base: for j in 0..MAX {
            for k in 0..=j {
                if t[j][k] == i {
                    r = j;
                    c = k;
                    break 'base;
                }
            }
        }

        while r > 0 {
            if r == c {
                if t[r][c] > t[r - 1][c - 1] {
                    break;
                }
                swap_in_vec(&mut t, r, c, r - 1, c - 1, &mut res);
                r -= 1;
                c -= 1;
            } else if c == 0 {
                if t[r][c] > t[r - 1][c] {
                    break;
                }
                swap_in_vec(&mut t, r, c, r - 1, c, &mut res);
                r -= 1;
            } else {
                if t[r][c] > t[r - 1][c] && t[r][c] > t[r - 1][c - 1] {
                    break;
                }

                if t[r][c] > t[r - 1][c] {
                    swap_in_vec(&mut t, r, c, r - 1, c - 1, &mut res);
                    r -= 1;
                    c -= 1;
                } else if t[r][c] > t[r - 1][c - 1] {
                    swap_in_vec(&mut t, r, c, r - 1, c, &mut res);
                    r -= 1;
                } else if t[r - 1][c] < t[r - 1][c - 1] && t[r - 1][c - 1] - t[r - 1][c] > 20 {
                    swap_in_vec(&mut t, r, c, r - 1, c - 1, &mut res);
                    r -= 1;
                    c -= 1;
                } else if t[r - 1][c] > t[r - 1][c - 1] && t[r - 1][c] - t[r - 1][c - 1] > 20 {
                    swap_in_vec(&mut t, r, c, r - 1, c, &mut res);
                    r -= 1;
                } else {
                    let d = rng.gen_bool(
                        t[r - 1][c - 1].pow(7) as f64
                            / (t[r - 1][c - 1].pow(7) + t[r - 1][c].pow(7)) as f64,
                    ) as usize;
                    swap_in_vec(&mut t, r, c, r - 1, c - d, &mut res);
                    r -= 1;
                    c -= d;
                }
            }
        }

        if res.len() == limit {
            return (res, t);
        }
    }
    (res, t)
}

fn main() {
    let tm = std::time::SystemTime::now();
    let mut t = vec![];
    for i in 1..=30 {
        input! {b: [usize; i]}
        t.push(b);
    }

    let mut res = naive(t.clone());

    while tm.elapsed().unwrap().as_millis() < 1900 {
        // let r = randomized(t.clone());
        let (r, _) = min_first(res.len(), t.clone());
        // eprintln!("res_t: {}", check(&res_t));
        // eprintln!("res: {}, r: {}", res.len(), r.len());
        if res.len() > r.len() {
            res = r;
        }

        if tm.elapsed().unwrap().as_millis() > 1950 {
            break;
        }
    }

    println!("{}", res.len());
    for (a, b, c, d) in res {
        println!("{} {} {} {}", a, b, c, d);
    }
}
