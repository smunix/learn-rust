use std::collections::{HashMap, HashSet};
use std::{fmt::Display, thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

#[derive(Clone, Copy)]
enum Op {
    LR(fn(char, String, String, char) -> ()),
    RL(fn(char, String, String, char) -> ()),
}

impl Op {
    pub fn apply(&self, pos: (usize, usize), delim: (char, char)) {
        match self {
            Op::LR(f) => f(
                delim.0,
                "*".repeat(pos.0),
                " ".repeat(pos.1 - pos.0),
                delim.1,
            ),
            Op::RL(f) => f(
                delim.0,
                "*".repeat(pos.1 - pos.0),
                " ".repeat(pos.0),
                delim.1,
            ),
        }
    }
}

struct Bounded {
    size: usize,
    delim: (char, char),
    dir: Op,
}

struct UnBounded {}

trait HasDisplayProgress {
    fn display_progress(&self, _: usize);
}

impl HasDisplayProgress for UnBounded {
    fn display_progress(&self, us: usize) {
        println!("{}{}", CLEAR, "*".repeat(us));
    }
}

impl HasDisplayProgress for Bounded {
    fn display_progress(&self, us: usize) {
        self.dir.apply((us, self.size), self.delim)
    }
}

struct Progress<Iter, B> {
    iter: Iter,
    i: usize,
    bound: B,
}

impl<Iter, B> Progress<Iter, B> {
    pub fn new(iter: Iter) -> Progress<Iter, UnBounded> {
        Progress {
            iter,
            i: 0,
            bound: UnBounded {},
        }
    }
}

impl<Iter> Progress<Iter, UnBounded>
where
    Iter: ExactSizeIterator,
{
    pub fn bounded(self) -> Progress<Iter, Bounded> {
        let iter = self.iter;
        let i = self.i;
        let size = iter.len();
        let delim = ('<', '>');
        let op_fn = |s, a, b, e| println!("{}{}{}{}{}", CLEAR, s, a, b, e);
        let dir = Op::LR(op_fn);
        let bound = Bounded { size, delim, dir };
        Progress { iter, i, bound }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delim(mut self, delim: (char, char)) -> Self {
        self.bound.delim = delim;
        self
    }
}

impl<Iter> DoubleEndedIterator for Progress<Iter, Bounded>
where
    Iter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.bound.dir = match self.bound.dir {
            Op::LR(op_fn) => Op::RL(op_fn),
            op => op,
        };
        self.bound.display_progress(self.i);
        self.i += 1;
        self.iter.next_back()
    }
}

impl<Iter, B> Iterator for Progress<Iter, B>
where
    Iter: Iterator,
    B: HasDisplayProgress,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.bound.display_progress(self.i);
        self.i += 1;
        self.iter.next()
    }
}

trait HasProgress
where
    Self: Sized,
{
    fn progress(self) -> Progress<Self, UnBounded>;
}

impl<Iter> HasProgress for Iter {
    fn progress(self) -> Progress<Self, UnBounded> {
        Progress::<Self, UnBounded>::new(self)
    }
}

trait HasCompute {
    fn compute(&self, d: Duration, pp: fn(&Self) -> ());
}

impl<T> HasCompute for &T
where
    T: Display,
{
    fn compute(&self, d: Duration, pp: fn(&Self) -> ()) {
        pp(self);
        sleep(d)
    }
}

impl<A, B> HasCompute for (A, B)
where
    A: HasCompute + Display,
    B: HasCompute + Display,
{
    fn compute(&self, d: Duration, pp: fn(&Self) -> ()) {
        pp(self);
        self.0.compute(d, |e| println!("->{}", e));
        self.1.compute(d, |e| println!("->{}", e));
    }
}

fn main() {
    let _i = 1.progress();
    let _s = "s".progress();
    let d = Duration::from_millis(250);
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    for n in vec.iter().progress().bounded() {
        n.compute(d, |i| println!("{}", i))
    }
    for n in vec.iter().progress().bounded().rev() {
        n.compute(d, |i| println!("{}", i))
    }
    for n in vec.iter().progress().bounded().rev().rev() {
        n.compute(d, |i| println!("{}", i))
    }
    let mut hs = HashSet::<i32>::new();
    for i in (1..10).rev() {
        hs.insert(i);
    }
    for n in hs.iter().progress().bounded().with_delim(('[', ']')) {
        n.compute(d, |i| println!("{}", i))
    }
    let mut hm: HashMap<i32, String> = HashMap::new();
    for i in (1..10).rev() {
        hm.insert(i, (i * 2).to_string());
    }
    for n in hm.iter().progress().bounded().with_delim(('{', '}')) {
        n.compute(d, |(i, s)| println!("({}x{})", i, s))
    }
}
