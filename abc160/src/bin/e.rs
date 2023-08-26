use proconio::*;

fn main() {
    input! {x: usize, y: usize, a: usize, b: usize, c: usize, p: [usize; a], q: [usize; b], r: [usize; c]}

    let mut buf = p
        .into_iter()
        .map(|v| (v, 0))
        .chain(
            q.into_iter()
                .map(|v| (v, 1))
                .chain(r.into_iter().map(|v| (v, 2))),
        )
        .collect::<Vec<_>>();
    buf.sort();

    let mut nocolored = 0;
    let mut red = 0;
    let mut blue = 0;
    let mut res = 0;
    while let Some((c, ty)) = buf.pop() {
        if ty == 0 {
            if red == x {
                continue;
            }
            red += 1;
        } else if ty == 1 {
            if blue == y {
                continue;
            }
            blue += 1;
        } else {
            nocolored += 1;
        }
        res += c;

        if nocolored + red + blue == x + y {
            break;
        }
    }

    println!("{}", res)
}
