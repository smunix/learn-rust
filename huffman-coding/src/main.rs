use std::collections::HashMap;
use std::hash::Hash;

mod huffman {
    use super::*;
    pub mod freq_of {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        use super::*;
        pub fn chars(lines: &Vec<String>) -> HashMap<char, u64> {
            lines
                .par_iter()
                .fold(
                    || HashMap::new(),
                    |mut frqs, line| {
                        for c in line.chars() {
                            *frqs.entry(c).or_insert(0) += 1;
                        }
                        frqs
                    },
                )
                .reduce(
                    || HashMap::new(),
                    |mut frqs1, frqs2| {
                        frqs2
                            .into_iter()
                            .for_each(|(c, n)| *frqs1.entry(c).or_insert(0) += n);
                        frqs1
                    },
                )
        }

        pub fn words(lines: &Vec<String>) -> HashMap<String, u64> {
            lines
                .par_iter()
                .fold(
                    || HashMap::new(),
                    |mut frqs, line| {
                        for w in line.split_ascii_whitespace() {
                            *frqs.entry(w.to_string()).or_insert(0) += 1;
                        }
                        frqs
                    },
                )
                .reduce(
                    || HashMap::new(),
                    |mut frqs1, frqs2| {
                        frqs2.into_iter().for_each(|(w, n)| {
                            *frqs1.entry(w).or_insert(0) += n;
                        });
                        frqs1
                    },
                )
        }
    }

    pub mod tree {
        use std::{
            cmp::Reverse,
            collections::{BinaryHeap, HashMap},
        };

        // huffman tree
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Tree<T> {
            Empty,
            Leaf {
                freq: u64,
                data: T,
            },
            Fork {
                freq: u64,
                children: (Box<Tree<T>>, Box<Tree<T>>),
            },
        }

        impl<T> Tree<T>
        where
            T: Clone,
        {
            pub fn data(&self) -> Option<T> {
                match self {
                    Self::Leaf { data, .. } => Some(data.clone()),
                    _ => None,
                }
            }

            pub fn freq(&self) -> u64 {
                match self {
                    Self::Leaf { freq, .. } => *freq,
                    Self::Fork { freq, .. } => *freq,
                    _ => 0,
                }
            }

            pub fn l(&self) -> Option<&Tree<T>> {
                match self {
                    Self::Fork { children, .. } => Some(&children.0),
                    _ => None,
                }
            }

            pub fn r(&self) -> Option<&Tree<T>> {
                match self {
                    Self::Fork { children, .. } => Some(&children.1),
                    _ => None,
                }
            }
        }

        pub fn mk<T>(freqs: &HashMap<T, u64>) -> Tree<T>
        where
            T: Clone,
            T: Eq,
        {
            let mut heap = BinaryHeap::new();

            for (t, n) in freqs {
                let (freq, data) = (*n, t.clone());
                heap.push(Reverse(Tree::<T>::Leaf { freq, data }))
            }

            while 1 < heap.len() {
                if let (Some(t1), Some(t2)) = (heap.pop(), heap.pop()) {
                    let freq = t1.0.freq() + t2.0.freq();
                    let children = (Box::new(t1.0), Box::new(t2.0));
                    heap.push(Reverse(Tree::<T>::Fork { freq, children }));
                }
            }

            if let Some(t) = heap.pop() {
                t.0
            } else {
                Tree::<T>::Empty
            }
        }
        impl<T> PartialOrd for Tree<T>
        where
            T: Eq,
            T: Clone,
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<T> Ord for Tree<T>
        where
            T: Clone,
            T: Eq,
        {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.freq().cmp(&other.freq())
            }
        }
    }

    pub mod codec {
        use bit_vec::BitVec;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Enc<T>(HashMap<T, BitVec>)
        where
            T: Eq,
            T: Hash;

        impl<T> Enc<T>
        where
            T: Eq,
            T: Hash,
        {
            pub fn get(&self, t: &T) -> Option<&BitVec> {
                self.0.get(t)
            }
        }

        impl<T> Enc<T>
        where
            T: Clone,
            T: Hash,
            T: Eq,
        {
            pub fn iso(&self) -> Dec<T> {
                let mut dec = HashMap::new();
                for (t, bv) in &self.0 {
                    dec.insert(bv.clone(), t.clone());
                }
                Dec(dec)
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Dec<T>(HashMap<BitVec, T>);

        impl<T> Dec<T> {
            pub fn get(&self, bv: &BitVec) -> Option<&T> {
                self.0.get(bv)
            }
        }
        use super::*;
        impl<T> tree::Tree<T>
        where
            T: Clone,
            T: Eq,
            T: Hash,
        {
            pub fn encoder(&self) -> Enc<T> {
                let mut enc = HashMap::new();
                let mut stack = vec![(self, BitVec::new())];

                while !stack.is_empty() {
                    if let Some((t, bv)) = stack.pop() {
                        match t {
                            Self::Empty => {}
                            Self::Leaf { data, .. } => {
                                enc.insert(data.clone(), bv.clone());
                            }
                            Self::Fork { children, .. } => {
                                stack.push((&children.0, {
                                    let mut bv = bv.clone();
                                    bv.push(false);
                                    bv
                                }));
                                stack.push((&children.1, {
                                    let mut bv = bv.clone();
                                    bv.push(true);
                                    bv
                                }));
                            }
                        }
                    }
                }
                Enc(enc)
            }
        }
    }

    pub mod compress {
        use bit_vec::BitVec;
        use huffman::tree;
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
        use serde::{Deserialize, Serialize};

        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Payload<T>
        where
            T: Eq,
            T: Hash,
        {
            codec: codec::Enc<T>,
            data: Vec<BitVec>,
        }

        impl<T> Payload<T>
        where
            T: Hash + Eq + Clone + Send + Sync,
        {
            pub fn compress<'a, Freqs, Tokens, TokensI>(
                freqs: Freqs,
                tokens: Tokens,
                lines: &'a Vec<String>,
            ) -> Payload<T>
            where
                Freqs: Fn(&'a Vec<String>) -> HashMap<T, u64>,
                Tokens: Fn(&'a str) -> TokensI + Sync,
                TokensI: Iterator<Item = T> + Send + Sync,
            {
                let counts = freqs(lines);
                let tree = tree::mk(&counts);
                let codec = tree.encoder();

                let data = lines
                    .par_iter()
                    .map(|line| {
                        tokens(line).map(|ref tk| codec.get(tk)).fold(
                            BitVec::new(),
                            |mut acc, bv| {
                                if let Some(bv) = bv {
                                    acc.extend(bv);
                                }
                                acc
                            },
                        )
                    })
                    .collect::<Vec<BitVec>>();

                // let data = lines
                //     .par_iter()
                //     .fold(
                //         || vec![BitVec::new()],
                //         |acc, line| {
                //             tokens(line)
                //                 .map(|ref tk| codec.get(tk))
                //                 .fold(acc, |mut acc, bv| {
                //                     if let (Some(bv), Some(mut bts)) = (bv, acc.pop()) {
                //                         bts.extend(bv);
                //                         acc.push(bts);
                //                     }
                //                     acc
                //                 })
                //         },
                //     )
                //     .reduce(
                //         || Vec::new(),
                //         |mut acc, bv| {
                //             acc.extend(bv);
                //             acc
                //         },
                //     );
                Payload { codec, data }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod freq_of {
        use super::*;

        #[test]
        fn freq_of_chars_works() {
            let input = vec!["this is an epic chap", "you can not escape getting rusty"]
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let counts = huffman::freq_of::chars(&input);
            dbg!(format!("{counts:?}"));
            assert_eq!(counts[&' '], 9);
            assert_eq!(counts[&'a'], 4);
            assert_eq!(counts[&'e'], 4);
            assert_eq!(counts[&'g'], 2);
        }

        #[test]
        fn freq_of_words_works() {
            let input = vec![
                "this is an epic rusty boy",
                "you can not escape getting rusty",
            ]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
            let counts = huffman::freq_of::words(&input);
            dbg!(format!("{counts:?}"));
            assert_eq!(counts[&"this".to_string()], 1);
            assert_eq!(counts.get("this"), Some(&1u64));
            assert_eq!(counts.get("getting"), Some(&1u64));
            assert_eq!(counts[&"rusty".to_string()], 2);
            assert_eq!(counts.get("rusty"), Some(&2u64));
        }
    }
    mod tree {
        use super::*;

        #[test]
        fn mk_works() {
            let mut freqs = HashMap::new();
            freqs.insert('a', 40);
            freqs.insert('b', 35);
            freqs.insert('c', 20);
            freqs.insert('d', 5);

            let tree = huffman::tree::mk(&freqs);

            assert_eq!(tree.freq(), 100);

            // 1 bit => most frequent
            assert_eq!(tree.l().and_then(|n| n.data()), Some('a'));
            assert_eq!(tree.l().map(|n| n.freq()), Some(40));
            assert_eq!(tree.r().map(|n| n.freq()), Some(60));

            // 2 bits => 2nd most frequent
            assert_eq!(
                tree.r().and_then(|t| t.r()).and_then(|t| t.data()),
                Some('b')
            );
            assert_eq!(tree.r().and_then(|t| t.r()).map(|t| t.freq()), Some(35));

            // 3 bits => the least frequent
            assert_eq!(
                tree.r()
                    .and_then(|t| t.l())
                    .and_then(|t| t.r())
                    .and_then(|t| t.data()),
                Some('c')
            );
            assert_eq!(
                tree.r()
                    .and_then(|t| t.l())
                    .and_then(|t| t.r())
                    .map(|t| t.freq()),
                Some(20)
            );
            assert_eq!(
                tree.r()
                    .and_then(|t| t.l())
                    .and_then(|t| t.l())
                    .and_then(|t| t.data()),
                Some('d')
            );
            assert_eq!(
                tree.r()
                    .and_then(|t| t.l())
                    .and_then(|t| t.l())
                    .map(|t| t.freq()),
                Some(5)
            );
        }
    }
    mod coding {
        use bit_vec::BitVec;

        use super::*;

        #[test]
        fn encoder_works() {
            let mut freqs = HashMap::new();
            freqs.insert('a', 40);
            freqs.insert('b', 35);
            freqs.insert('c', 20);
            freqs.insert('d', 5);

            let tree = huffman::tree::mk(&freqs);
            let enc = tree.encoder();

            fn bit_vec(s: &str) -> BitVec {
                let mut bv = BitVec::new();
                for c in s.chars() {
                    match c {
                        '0' => {
                            bv.push(false);
                        }
                        '1' => {
                            bv.push(true);
                        }
                        _ => {}
                    }
                }
                bv
            }

            assert_eq!(tree.freq(), 100);
            assert_eq!(enc.get(&'a'), Some(&bit_vec("0")));
            assert_eq!(enc.get(&'b'), Some(&bit_vec("11")));
            assert_eq!(enc.get(&'c'), Some(&bit_vec("101")));
            assert_eq!(enc.get(&'d'), Some(&bit_vec("100")));
        }

        #[test]
        fn decoder_works() {
            let mut freqs = HashMap::new();
            freqs.insert('a', 40);
            freqs.insert('b', 35);
            freqs.insert('c', 20);
            freqs.insert('d', 5);

            let tree = huffman::tree::mk(&freqs);
            let dec = tree.encoder().iso();

            fn bit_vec(s: &str) -> BitVec {
                let mut bv = BitVec::new();
                for c in s.chars() {
                    match c {
                        '0' => {
                            bv.push(false);
                        }
                        '1' => {
                            bv.push(true);
                        }
                        _ => {}
                    }
                }
                bv
            }

            assert_eq!(tree.freq(), 100);
            assert_eq!(dec.get(&bit_vec("0")), Some(&'a'));
            assert_eq!(dec.get(&bit_vec("11")), Some(&'b'));
            assert_eq!(dec.get(&bit_vec("101")), Some(&'c'));
            assert_eq!(dec.get(&bit_vec("100")), Some(&'d'));
        }
    }
}

fn main() {
    {
        let vs = vec![
            "Hello, world!".to_string(),
            "hello, folks!".to_string(),
            "hello, world!".to_string(),
            "hello there!".to_string(),
        ];
        let compressed = huffman::compress::Payload::<char>::compress(
            huffman::freq_of::chars,
            |line| line.chars(),
            &vs,
        );
        println!("{compressed:?}");
    }
    // {
    //     let vs = vec!["allo".to_string(), "hello".to_string(), "alo".to_string()];
    //     let tree = huffman::tree::mk(&huffman::freq_of::chars(&vs));
    //     let enc = tree.encoder();
    //     println!("{enc:?}");
    //     let compressed = huffman::compress::Payload::<char>::new(
    //         huffman::freq_of::chars,
    //         |line| line.chars(),
    //         &vs,
    //     );
    //     println!("{compressed:?}");
    // }
    // {
    //     let tree = huffman::tree::mk(&huffman::freq_of::chars(&vec![]));
    //     let enc = tree.encoder();
    //     println!("{enc:?}");
    //     let compressed = huffman::compress::Payload::<char>::new(
    //         huffman::freq_of::chars,
    //         |line| line.chars(),
    //         &vec!["hey people! be nice!".to_string()],
    //     );
    //     println!("{compressed:?}");
    // }
    // {
    //     let tree = huffman::tree::mk(&huffman::freq_of::words(&vec![
    //         "allo".to_string(),
    //         "hello".to_string(),
    //         "alo".to_string(),
    //     ]));
    //     let enc = tree.encoder();
    //     println!("{enc:?}");
    //     let compressed = huffman::compress::Payload::<char>::new(
    //         huffman::freq_of::chars,
    //         |line| line.chars(),
    //         &vec!["hey people! be nice!".to_string()],
    //     );
    //     println!("{compressed:?}");
    // }
}
