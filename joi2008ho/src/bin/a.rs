use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut stack = vec![];
    let mut cnt = 0;
    for c in a {
        if cnt % 2 == 0 {
            match stack.last_mut() {
                Some((p, cnt)) if *p == c => *cnt += 1,
                _ => stack.push((c, 1)),
            }
        } else if let Some((p, cnt)) = stack.pop() {
            if p == c {
                stack.push((p, cnt + 1));
            } else if let Some((_, pcnt)) = stack.last_mut() {
                *pcnt += cnt + 1;
            } else {
                stack.push((c, cnt + 1));
            }
        } else {
            stack.push((c, 1));
        }
        cnt += 1;
    }

    println!(
        "{}",
        stack
            .into_iter()
            .filter_map(|(p, cnt)| (p == 0).then_some(cnt))
            .sum::<usize>()
    );
}
