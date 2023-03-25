use proconio::input;

fn blue(
    n: usize,
    num: usize,
    x: usize,
    y: usize,
    memo: &mut std::collections::HashMap<(usize, char), usize>,
) -> usize {
    if n == 1 {
        return num;
    }
    if let Some(res) = memo.get(&(n, 'B')) {
        return num * *res;
    }

    let res = red(n - 1, 1, x, y, memo) + blue(n - 1, y, x, y, memo);
    memo.insert((n, 'B'), res);
    res * num
}

fn red(
    n: usize,
    num: usize,
    x: usize,
    y: usize,
    memo: &mut std::collections::HashMap<(usize, char), usize>,
) -> usize {
    if n == 1 {
        return 0;
    }
    if let Some(res) = memo.get(&(n, 'R')) {
        return num * *res;
    }

    let res = red(n - 1, 1, x, y, memo) + blue(n, x, x, y, memo);
    memo.insert((n, 'R'), res);
    res * num
}

fn main() {
    input! {n: usize, x: usize, y: usize}

    let mut memo = std::collections::HashMap::new();
    println!("{}", red(n, 1, x, y, &mut memo))
}
