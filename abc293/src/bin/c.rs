use proconio::*;

fn solve(r: usize, c: usize, stack: &mut Vec<usize>, a: &Vec<Vec<usize>>) -> usize {
    if r == a.len() - 1 && c == a[0].len() - 1 {
        return 1;
    }
    stack.push(a[r][c]);

    let mut res = 0;

    if r + 1 < a.len() && !stack.contains(&a[r + 1][c]) {
        res += solve(r + 1, c, stack, a);
    }
    if c + 1 < a[0].len() && !stack.contains(&a[r][c + 1]) {
        res += solve(r, c + 1, stack, a);
    }

    stack.pop().unwrap();

    res
}

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    let mut stack = vec![];
    println!("{}", solve(0, 0, &mut stack, &a));
}
