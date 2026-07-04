use proconio::*;

fn solve(target: usize, query: &[(u8, usize, usize, String)], context: &mut Vec<String>) -> String {
    let mut now = query.len();
    let mut buf = "".to_owned();
    while now > 0 {
        now -= 1;
        let (ty, tar, src, s) = &query[now];
        if *tar == target {
            buf = if *ty == 1 {
                solve(*src, &query[..now], &mut vec![])
            } else {
                context.push(s.clone());
                solve(*tar, &query[..now], context)
            };
            break;
        }
    }
    while let Some(ctx) = context.pop() {
        buf.push_str(&ctx);
    }
    buf
}

fn main() {
    input! {n: usize, q: usize}

    // (type, target, source, string)
    let mut query = vec![];
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {p: usize}
            query.push((1, p, n + 1, "".to_owned()));
        } else if ty == 2 {
            input! {p: usize, s: String}
            query.push((2, p, 0, s));
        } else {
            input! {p: usize}
            query.push((1, n + 1, p, "".to_owned()));
        }
    }

    println!("{}", solve(n + 1, &query, &mut vec![]));
}
