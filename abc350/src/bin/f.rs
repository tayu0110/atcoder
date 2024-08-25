use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let n = s.len();

    let mut sp = s.iter().cloned().collect::<SplayTree>();
    let mut stack = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            let prev = stack.pop().unwrap();
            sp.reverse(prev..i).unwrap();
        }
    }

    let mut res = String::new();
    for i in 0..n {
        let c = sp.get(i);
        if c != '(' && c != ')' {
            res.push(c);
        }
    }

    println!("{res}")
}

use std::cell::Cell;
use std::ops::{Deref, DerefMut, Range};
use std::ptr::NonNull;

#[derive(Clone, Copy)]
struct Node {
    parent: Option<NodeRef>,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    val: i16,
    subsum: u32,
    rev: bool,
}

impl Node {
    pub const fn is_reversed(&self) -> bool {
        self.rev
    }

    pub fn toggle(&mut self) {
        self.rev ^= true;
        self.val = -self.val;
        (self.left, self.right) = (self.right, self.left);
    }

    fn propagate(&mut self) {
        if self.is_reversed() {
            if let Some(mut left) = self.left {
                left.toggle();
            }
            if let Some(mut right) = self.right {
                right.toggle();
            }
        }

        self.rev = false;
    }

    fn update(&mut self) {
        self.subsum = 1;
        if let Some(l) = self.left {
            self.subsum += l.subsum;
        }
        if let Some(r) = self.right {
            self.subsum += r.subsum;
        }
    }
}

struct NodeRef(NonNull<Node>);

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for NodeRef {}

impl NodeRef {
    fn new(val: char) -> Self {
        let ptr = Box::leak(Box::new(Node {
            parent: None,
            left: None,
            right: None,
            val: val as i16,
            subsum: 1,
            rev: false,
        }));
        Self(NonNull::new(ptr).unwrap())
    }

    unsafe fn into_raw(self) -> Node {
        let raw = unsafe { Box::from_raw(self.0.as_ptr()) };
        *raw
    }

    fn connect_left(mut self, mut child: Self) {
        child.parent = Some(self);
        self.left = Some(child);
    }

    fn disconnect_left(mut self) -> Option<Self> {
        let mut left = self.left?;
        left.parent = None;
        self.left = None;
        Some(left)
    }

    fn is_left_child(self) -> bool {
        self.parent
            .map_or(false, |p| p.left.map(|p| p.0) == Some(self.0))
    }

    fn connect_right(mut self, mut child: Self) {
        child.parent = Some(self);
        self.right = Some(child);
    }

    fn disconnect_right(mut self) -> Option<Self> {
        let mut right = self.right?;
        right.parent = None;
        self.right = None;
        Some(right)
    }

    fn is_right_child(self) -> bool {
        self.parent
            .map_or(false, |p| p.right.map(|p| p.0) == Some(self.0))
    }

    fn disconnect_parent(self) -> Option<Self> {
        let par = self.parent?;

        if self.is_left_child() {
            par.disconnect_left();
            Some(par)
        } else if self.is_right_child() {
            par.disconnect_right();
            Some(par)
        } else {
            unreachable!()
        }
    }

    fn rotate_left(mut self) -> Self {
        let Some(mut right) = self.disconnect_right() else {
            return self;
        };
        right.subsum = self.subsum;

        if let Some(right_left) = right.disconnect_left() {
            self.connect_right(right_left);
        }

        self.update();

        let self_is_left = self.is_left_child();
        let par = self.disconnect_parent();
        right.connect_left(self);

        if let Some(par) = par {
            if self_is_left {
                par.connect_left(right);
            } else {
                par.connect_right(right);
            }
        }

        // return new root of the original subtree.
        right
    }

    fn rotate_right(mut self) -> Self {
        let Some(mut left) = self.disconnect_left() else {
            return self;
        };
        left.subsum = self.subsum;

        if let Some(left_right) = left.disconnect_right() {
            self.connect_left(left_right);
        }

        self.update();

        let self_is_left = self.is_left_child();
        let par = self.disconnect_parent();
        left.connect_right(self);

        if let Some(par) = par {
            if self_is_left {
                par.connect_left(left);
            } else {
                par.connect_right(left);
            }
        }

        // return new root of the original subtree
        left
    }

    fn splay(mut self) {
        self.propagate();
        while !self.is_root() {
            let &parent = self.parent.as_ref().unwrap();

            if parent.is_root() {
                // zig step
                if self.is_left_child() {
                    parent.rotate_right();
                } else {
                    parent.rotate_left();
                }
                return;
            }

            let &grand_parent = parent.parent.as_ref().unwrap();
            if self.is_left_child() ^ parent.is_left_child() {
                // zig-zag step
                if self.is_left_child() {
                    parent.rotate_right();
                    grand_parent.rotate_left();
                } else if self.is_right_child() {
                    parent.rotate_left();
                    grand_parent.rotate_right();
                } else {
                    unreachable!()
                }
            } else {
                // zig-zig step
                if self.is_left_child() {
                    grand_parent.rotate_right();
                    parent.rotate_right();
                } else if self.is_right_child() {
                    grand_parent.rotate_left();
                    parent.rotate_left();
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn is_root(self) -> bool {
        self.parent.map_or(true, |p| {
            p.left.map_or(true, |s| s.0 != self.0) && p.right.map_or(true, |d| d.0 != self.0)
        })
    }
}

impl Deref for NodeRef {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for NodeRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

struct SplayTree {
    root: Option<Cell<NodeRef>>,
}

impl SplayTree {
    const fn new() -> Self {
        Self { root: None }
    }

    fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |c| c.get().subsum as usize)
    }

    fn nth_node(&self, mut index: usize) -> Option<NodeRef> {
        if index >= self.len() {
            return None;
        }

        index += 1;
        let mut node = self.root.as_ref()?.get();
        let mut seen = 0;
        while seen < index {
            node.propagate();
            if let Some(left) = node.left {
                if left.subsum as usize + seen >= index {
                    node = left;
                    continue;
                }

                seen += left.subsum as usize;
            }

            seen += 1;
            if seen == index {
                break;
            }

            let Some(right) = node.right else { break };
            node = right;
        }
        node.propagate();
        node.splay();
        self.root.as_ref().unwrap().set(node);

        Some(node)
    }

    fn get(&self, index: usize) -> char {
        let nth = self.nth_node(index).unwrap();
        let sgn = nth.val.signum();
        let c = char::from_u32(nth.val.abs() as u32).unwrap();
        if sgn < 0 {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        } else {
            c
        }
    }

    fn insert(&mut self, index: usize, val: char) -> Result<(), &'static str> {
        if self.len() == 0 {
            let new = NodeRef::new(val);
            self.root = Some(Cell::new(new));
            return Ok(());
        }
        if index == self.len() {
            let new = NodeRef::new(val);
            let mut node = self.nth_node(self.len() - 1).unwrap();
            node.splay();
            node.connect_right(new);
            node.update();

            self.root.as_mut().unwrap().set(node);
            return Ok(());
        }

        let back = self.split_off(index)?;
        let mut new = NodeRef::new(val);
        new.connect_left(self.root.as_ref().unwrap().get());
        self.root.as_ref().unwrap().set(new);
        new.update();
        self.extend(back);

        Ok(())
    }

    fn remove(&mut self, index: usize) -> Option<char> {
        let mut back = self.split_off(index).ok()?;

        let node = back.nth_node(0)?;
        if let Some(right) = node.disconnect_right() {
            back.root.as_ref().unwrap().set(right);
        } else {
            back.root = None;
        }

        self.extend(back);

        Some(unsafe { char::from_u32(node.into_raw().val as u32)? })
    }

    fn split_off(&mut self, at: usize) -> Result<Self, &'static str> {
        if at == 0 {
            let mut res = Self::new();
            std::mem::swap(self, &mut res);
            return Ok(res);
        } else if at == self.len() {
            return Ok(Self::new());
        }

        let mut split_point = self
            .nth_node(at)
            .ok_or("index out of range in SplayTree::split_off")?;
        split_point.propagate();
        let left = split_point.disconnect_left().unwrap();
        split_point.update();

        let res = Self {
            root: Some(Cell::new(split_point)),
        };

        self.root.as_mut().unwrap().set(left);
        Ok(res)
    }

    fn extend(&mut self, other: Self) {
        if other.root.is_none() {
            return;
        } else if self.root.is_none() {
            self.root = other.root;
            return;
        }

        let mut node = self.nth_node(self.len() - 1).unwrap();

        node.connect_right(other.root.unwrap().get());
        node.update();
        self.root.as_mut().unwrap().set(node);
    }

    fn reverse(&mut self, range: Range<usize>) -> Result<(), &'static str> {
        if range.is_empty() {
            return Ok(());
        }

        let Range { start, end } = range;
        let back = self.split_off(end)?;
        let mut mid = self.split_off(start)?;

        let mut mid_root = mid.root.as_mut().unwrap().get();
        mid_root.toggle();
        mid_root.propagate();

        self.extend(mid);
        self.extend(back);

        Ok(())
    }

    fn set(&mut self, index: usize, val: char) -> Result<(), &'static str> {
        let mut node = self
            .nth_node(index)
            .ok_or("index out of range in SplayTree::set")?;
        node.val = val as i16;
        node.update();
        self.root.as_mut().unwrap().set(node);
        Ok(())
    }
}

impl FromIterator<char> for SplayTree {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let Some(mut node) = iter.next().map(NodeRef::new) else {
            return SplayTree::new();
        };

        for c in iter {
            let new = NodeRef::new(c);
            node.connect_right(new);
            node = new;
        }

        while let Some(mut par) = node.parent {
            par.update();
            node = par;
        }

        SplayTree {
            root: Some(Cell::new(node)),
        }
    }
}
