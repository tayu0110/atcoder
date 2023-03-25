use proconio::input;

fn rec(k: usize, a: &[usize], memo: &mut Vec<u8>) -> u8 {
    if memo[k] != 0 {
        return 3 - memo[k];
    }

    for na in a {
        if *na > k {
            break;
        }

        if rec(k - na, a, memo) == 1 {
            memo[k] = 1;
            return 2;
        }
    }

    memo[k] = 2;
    1
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    // 0: undecided, 1: win, 2: lose
    let mut memo = vec![0u8; k + 1];
    if rec(k, &a, &mut memo) == 2 {
        println!("First")
    } else {
        println!("Second")
    }
}
