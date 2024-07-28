use std::fmt::Debug;

trait Sort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Debug,
        T: Ord;
}

mod bubble {
    use super::*;

    pub struct Algo;

    impl Sort for Algo {
        fn sort<T>(&self, vs: &mut [T])
        where
            T: Ord,
        {
            match vs.len() {
                0 => return,
                _ => {}
            }

            let mut swapped = true;
            while swapped {
                swapped = false;
                for i in 0..(vs.len() - 1) {
                    if vs[i] > vs[i + 1] {
                        vs.swap(i, i + 1);
                        swapped = true;
                    }
                }
            }
        }
    }
}

mod insertion {

    use super::*;

    pub struct Algo;

    impl Sort for Algo {
        fn sort<T>(&self, vs: &mut [T])
        where
            T: Ord,
        {
            // [ sort | not sort ]
            for unsort in 1..vs.len() {
                let sort = vs[..unsort]
                    .binary_search(&vs[unsort])
                    .map_or_else(|x| x, |x| x);
                vs[sort..=unsort].rotate_right(1);
            }
        }
    }
}

mod selection {

    use super::*;

    pub struct Algo;

    impl Sort for Algo {
        fn sort<T>(&self, vs: &mut [T])
        where
            T: Ord,
        {
            // [ sort | not sort ]
            for unsorted in 0..vs.len() {
                let (smallest_in_rest, _) = vs[unsorted..]
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, v)| v)
                    .expect("slice isn't empty");
                let smallest_in_rest = unsorted + smallest_in_rest;

                if unsorted != smallest_in_rest {
                    vs.swap(unsorted, smallest_in_rest);
                }
            }
        }
    }
}

mod quicksort {

    use super::*;

    pub struct Algo;

    impl Sort for Algo {
        fn sort<T>(&self, vs: &mut [T])
        where
            T: Ord,
            T: Debug,
        {
            fn inc(x: &mut usize) {
                *x += 1;
            }

            fn dec(x: &mut usize) {
                *x -= 1;
            }

            fn shift(msg: &str, fun: fn(&mut usize), pos: &mut usize) {
                #[cfg(test)]
                print!("\t{msg}: {pos}");
                fun(pos);
                #[cfg(test)]
                println!(" => {pos}");
            }

            fn swap_lr<T>(
                cond: impl Fn() -> bool,
                pivot: &mut T,
                rest: &mut [T],
                left: &mut usize,
                right: &mut usize,
            ) where
                T: Debug,
            {
                #[cfg(test)]
                println!("\tswap_lr({pivot:?}, {rest:?}, {left}, {right})?");
                if cond() {
                    rest.swap(*left, *right);
                    shift("\tleft", inc, left);
                    shift("\tright", dec, right);
                }
                #[cfg(test)]
                println!("\t... => {rest:?}");
            }

            #[cfg(test)]
            println!("quicksort({:?})", vs);
            match vs.len() {
                0 | 1 => {
                    #[cfg(test)]
                    println!("...  => {:?}", vs);
                    return;
                }

                2 => {
                    if vs[0] > vs[1] {
                        vs.swap(0, 1);
                        #[cfg(test)]
                        println!("...  => {:?}", vs);
                        return;
                    }
                }
                _ => {}
            }
            let (pivot, rest) = vs.split_first_mut().expect("slice is non-empty");
            let mut left = 0;
            let mut right = rest.len() - 1;

            while left < right {
                #[cfg(test)]
                println!("pivot={pivot:?}, rest={rest:?}, bound=({left},{right})");
                if &rest[left] < pivot {
                    shift("left", inc, &mut left);
                } else if &rest[right] > pivot {
                    shift("right", dec, &mut right);
                } else {
                    swap_lr(|| true, pivot, rest, &mut left, &mut right);
                }
                #[cfg(test)]
                println!("... => rest={rest:?}, bound=({left},{right})");
            }

            if left == right {
                if &rest[left] < pivot {
                    self.sort(&mut rest[left - 1..left + 1]);
                    shift("left", inc, &mut left);
                }
            }

            // place the pivot at its final location
            vs.swap(0, left);
            #[cfg(test)]
            println!("...  => {:?}", vs);

            let (left, right) = vs.split_at_mut(left);
            self.sort(left);
            self.sort(&mut right[1..]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_it<Algo>(go: &Algo)
    where
        Algo: Sort,
    {
        let mut vecs = vec![
            vec![],
            vec![2, 1],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![5, 1, 3, 2, 4],
            vec![5, 1, 2, 4, 3],
            vec![4, 3, 5],
            vec![5, 4, 3, 5],
            vec![6, 4, 3, 5],
            vec![1, 4, 3, 5],
        ];
        for xs in &mut vecs {
            let ys = {
                let mut xs = xs.clone();
                xs.sort();
                xs
            };
            go.sort(xs);
            assert_eq!(xs, &ys)
        }
    }

    #[test]
    fn test_bubble_works() {
        test_it(&bubble::Algo);
    }

    #[test]
    fn test_insertion_works() {
        test_it(&insertion::Algo);
    }

    #[test]
    fn test_selection_works() {
        test_it(&selection::Algo);
    }

    #[test]
    fn test_quicksort_works() {
        test_it(&quicksort::Algo);
    }
}

mod bench {
    use super::*;
    use std::{
        cell::Cell,
        rc::Rc,
        time::{Duration, Instant},
    };

    use rand::Rng;

    use crate::Sort;

    #[derive(Debug, Clone)]
    struct Eval<T> {
        t: T,
        cmps: Rc<Cell<usize>>,
    }

    impl<T> Eq for Eval<T> where T: Eq {}

    impl<T> PartialEq for Eval<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.t == other.t
        }
    }
    impl<T> PartialOrd for Eval<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.cmps.set(self.cmps.get() + 1);
            self.t.partial_cmp(&other.t)
        }
    }
    impl<T> Ord for Eval<T>
    where
        T: Ord,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    fn bench<T, A>(algo: A, values: &[Eval<T>], counter: &Cell<usize>) -> (usize, Duration)
    where
        T: Ord,
        T: Clone,
        T: Debug,
        A: Sort,
    {
        let mut values: Vec<_> = values.to_vec();
        counter.set(0);
        let took = Instant::now();
        algo.sort(&mut values);
        let took = took.elapsed();
        let count = counter.get();
        for i in 1..values.len() {
            assert!(values[i - 1] <= values[i]);
        }
        (count, took)
    }

    pub fn run() {
        let mut rand = rand::thread_rng();
        let counter = Rc::new(Cell::new(0));
        for &n in &[0, 10, 100, 1000, 10000, 100000] {
            for _ in 0..1 {
                let mut values = Vec::with_capacity(n);
                for _ in 0..n {
                    values.push(Eval {
                        t: rand.gen::<usize>(),
                        cmps: Rc::clone(&counter),
                    });
                }

                println!("{}", "*".repeat(50));

                {
                    let took = bench(crate::bubble::Algo, &values, &counter);
                    println!("bubble {n} {took:?}");
                }
                {
                    let took = bench(crate::insertion::Algo, &values, &counter);
                    println!("insertion {n} {took:?}");
                }
                {
                    let took = bench(crate::selection::Algo, &values, &counter);
                    println!("selection {n} {took:?}");
                }
                {
                    let took = bench(crate::quicksort::Algo, &values, &counter);
                    println!("quicksort {n} {took:?}");
                }
            }
        }
    }
}

fn main() {
    bench::run();
}
