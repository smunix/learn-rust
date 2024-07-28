use std::fmt::Debug;

trait Sort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Debug,
        T: Ord;
}

mod bubble {
    use super::*;

    pub struct Go;

    impl Sort for Go {
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

    pub struct Go;

    impl Sort for Go {
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

    pub struct Go;

    impl Sort for Go {
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

    pub struct Go;

    impl Sort for Go {
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
                print!("\t{msg}: {pos}");
                fun(pos);
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
                println!("\tswap_lr({pivot:?}, {rest:?}, {left}, {right})?");
                if cond() {
                    rest.swap(*left, *right);
                    shift("\tleft", inc, left);
                    shift("\tright", dec, right);
                }
                println!("\t... => {rest:?}");
            }

            println!("quicksort({:?})", vs);
            match vs.len() {
                0 | 1 => {
                    println!("...  => {:?}", vs);
                    return;
                }

                2 => {
                    if vs[0] > vs[1] {
                        vs.swap(0, 1);
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
                println!("pivot={pivot:?}, rest={rest:?}, bound=({left},{right})");
                if &rest[left] < pivot {
                    shift("left", inc, &mut left);
                } else if &rest[right] > pivot {
                    shift("right", dec, &mut right);
                } else {
                    swap_lr(|| true, pivot, rest, &mut left, &mut right);
                }
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
    fn test_it<Go>(go: &Go)
    where
        Go: Sort,
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
        test_it(&bubble::Go);
    }

    #[test]
    fn test_insertion_works() {
        test_it(&insertion::Go);
    }

    #[test]
    fn test_selection_works() {
        test_it(&selection::Go);
    }

    #[test]
    fn test_quicksort_works() {
        test_it(&quicksort::Go);
    }
}

fn main() {
    println!("Hello, world!");
}
