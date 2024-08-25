use proconio::*;

fn make_tree(s: &str, tree: &mut Vec<[(usize, usize); 2]>) -> (usize, usize) {
    if s.is_empty() {
        return (usize::MAX, usize::MAX);
    }

    let mut open = 1;
    for (i, c) in s.chars().enumerate().skip(1) {
        if c == '(' {
            open += 1;
        } else if c == ')' {
            open -= 1;
        }

        if open == 0 {
            let (left, mut rest) = s.split_at(i + 1);
            rest = rest.strip_prefix('[').unwrap();
            let (index, right) = rest.split_once(']').unwrap();
            let index = index.parse::<usize>().unwrap();
            let pos = tree.len();
            tree.push([(usize::MAX, usize::MAX); 2]);
            let left = make_tree(
                left.strip_prefix('(').unwrap().strip_suffix(')').unwrap(),
                tree,
            );
            let right = make_tree(
                right.strip_prefix('(').unwrap().strip_suffix(')').unwrap(),
                tree,
            );
            tree[pos] = [left, right];

            return (index, pos);
        }
    }

    unreachable!()
}

fn dump(now: usize, a: usize, b: usize, ta: &[[(usize, usize); 2]], tb: &[[(usize, usize); 2]]) {
    print!("(");
    if ta[a][0].0 < usize::MAX && tb[b][0].0 < usize::MAX {
        dump(ta[a][0].0 + tb[b][0].0, ta[a][0].1, tb[b][0].1, ta, tb);
    }
    print!(")[{}](", now);
    if ta[a][1].0 < usize::MAX && tb[b][1].0 < usize::MAX {
        dump(ta[a][1].0 + tb[b][1].0, ta[a][1].1, tb[b][1].1, ta, tb);
    }
    print!(")");
}

fn main() {
    input! {a: String, b: String}

    let (mut ta, mut tb) = (vec![], vec![]);
    let (wa, _) = make_tree(&a, &mut ta);
    let (wb, _) = make_tree(&b, &mut tb);

    dump(wa + wb, 0, 0, &ta, &tb);
    println!();
}
