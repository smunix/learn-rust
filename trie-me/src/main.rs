use std::collections::VecDeque;
use std::fmt::Display;

use node::Node;

#[derive(Debug)]
pub enum Error {}

mod node {
    #[derive(Debug, Default)]
    pub struct Node {
        pub children: Vec<Node>,
        pub key: Option<char>,
        pub val: Option<String>,
        pub count: usize,
    }

    impl Node {
        pub fn new() -> Self {
            Node {
                ..Default::default()
            }
        }
        pub fn with_key(c: char) -> Self {
            Node {
                key: Some(c),
                ..Default::default()
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Trie {
    pub root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            ..Default::default()
        }
    }
    pub fn insert(&mut self, s: &str) {
        let mut cur = &mut self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|n| n.key.cmp(&Some(c))) {
                Ok(i) => {
                    cur = &mut cur.children[i];
                }
                Err(i) => {
                    cur.children.insert(i, Node::with_key(c));
                    cur = &mut cur.children[i];
                }
            }
        }

        cur.count += 1;
        cur.val.replace(s.to_string());
    }
    pub fn exists<'a>(&'a self, s: &str) -> Option<&'a Node> {
        let mut cur = &self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|n| n.key.cmp(&Some(c))) {
                Ok(i) => {
                    cur = &cur.children[i];
                }
                Err(_) => {
                    return None;
                }
            }
        }

        if cur.count > 0 {
            Some(cur)
        } else {
            None
        }
    }
    pub fn search(&self, s: &str) -> Vec<String> {
        let mut cur = &self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|n| n.key.cmp(&Some(c))) {
                Ok(i) => {
                    cur = &cur.children[i];
                }
                Err(_) => return Default::default(),
            }
        }
        let mut results = Vec::new();
        let mut q = VecDeque::new();
        q.push_front(cur);
        while let Some(n) = q.pop_front() {
            for child in &n.children {
                q.push_front(child);
            }
            if n.count > 0 {
                results.push((n.count, n.val.as_ref()));
            }
        }
        results.sort_by(|a, b| a.cmp(b));
        results.iter().map(|(_, b)| b.unwrap().clone()).collect()
    }
}

impl Display for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut q = VecDeque::new();
        q.push_back(&self.root);

        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    for child in &node.children {
                        let r = write!(f, "{} ", &child.key.unwrap());
                        if r.is_err() {
                            return r;
                        }
                        if child.children.len() > 0 {
                            q.push_back(&child);
                        }
                    }
                }
            }

            if q.len() > 0 {
                let r = writeln!(f);
                if r.is_err() {
                    return r;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let mut trie = Trie::new();
        trie.insert("potatoe");
        trie.insert("province");
        trie.insert("profile");
        trie.insert("profileur");
        trie.insert("profiteur");
        trie.insert("prof");
        trie.insert("professor");
        trie.insert("provinces");
        trie.insert("providence");
        trie.insert("providences");

        dbg!(trie.search("prof"));
        println!("{trie}");
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
