use std::collections::{BTreeMap, BTreeSet};

use ds::{DefaultZST, EulerTourTree};
use proconio::*;

fn main() {
    input! {q: usize, k: usize, query: [(usize, usize); q]}

    let mut comp = BTreeMap::new();
    for (_, x) in &query {
        comp.insert(*x, 0);
    }
    let mut cnt = 0;
    for (_, x) in comp.iter_mut() {
        *x = cnt;
        cnt += 1;
    }
    let reverse = comp.iter().map(|v| *v.0).collect::<Vec<_>>();

    let mut in_tree = BTreeSet::new();
    let mut lct = EulerTourTree::<DefaultZST>::new(cnt);
    for (ty, x) in query {
        let index = *comp.get(&x).unwrap();
        if ty == 1 {
            if in_tree.remove(&index) {
                match (
                    in_tree.range(..index).next_back(),
                    in_tree.range(index..).next(),
                ) {
                    (Some(&prev), Some(&next)) => {
                        if lct.are_connected(prev, index) {
                            lct.cut(prev, index);
                        }
                        if lct.are_connected(index, next) {
                            lct.cut(index, next);
                        }
                        if reverse[prev].abs_diff(reverse[next]) <= k {
                            lct.link(prev, next).ok();
                        }
                    }
                    (Some(&prev), _) => {
                        if lct.are_connected(prev, index) {
                            lct.cut(prev, index);
                        }
                    }
                    (_, Some(&next)) => {
                        if lct.are_connected(index, next) {
                            lct.cut(index, next);
                        }
                    }
                    _ => {}
                }
            } else {
                match (
                    in_tree.range(..index).next_back(),
                    in_tree.range(index..).next(),
                ) {
                    (Some(&prev), Some(&next)) => {
                        if lct.are_connected(prev, next) {
                            lct.cut(prev, next);
                            lct.link(prev, index).ok();
                            lct.link(index, next).ok();
                        } else {
                            if reverse[prev].abs_diff(x) <= k {
                                lct.link(prev, index).ok();
                            }
                            if reverse[next].abs_diff(x) <= k {
                                lct.link(index, next).ok();
                            }
                        }
                    }
                    (Some(&prev), None) => {
                        if reverse[prev].abs_diff(x) <= k {
                            lct.link(prev, index).ok();
                        }
                    }
                    (None, Some(&next)) => {
                        if reverse[next].abs_diff(x) <= k {
                            lct.link(index, next).ok();
                        }
                    }
                    _ => {}
                }
                in_tree.insert(index);
            }
        } else {
            println!("{}", lct.tree_size(index));
        }
    }
}
