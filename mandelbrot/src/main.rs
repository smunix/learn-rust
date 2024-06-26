use num::Complex;
use std::str::FromStr;

fn openCloseP(s: &str, l: char, r: char) -> Option<&str> {
    let t = s.trim();
    match t.find(l) {
        Some(0) => match t.find(r) {
            Some(rix) => {
                if rix == t.len() - 1 {
                    Some(&t[1..rix])
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}

fn bracesP(s: &str) -> Option<&str> {
    openCloseP(s, '{', '}')
}

fn parensP(s: &str) -> Option<&str> {
    openCloseP(s, '(', ')')
}

fn pairP<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(lhs), Ok(rhs)) => Some((lhs, rhs)),
            _ => None,
        },
    }
}

#[test]
fn test_pairP() {
    assert_eq!(pairP::<i32>("", ','), None);
    assert_eq!(pairP::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(pairP::<i32>("10x20", 'x'), Some((10, 20)));
    assert_eq!(pairP::<f32>("10x20", 'x'), Some((10.0, 20.0)));
}

#[test]
fn test_complexP() {
    assert_eq!(
        complexP::<f64>("{10.0,20.0}"),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complexP::<f64>("   {10.0,20.0}"),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complexP::<f64>("{10.0,20.0}   "),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(complexP::<i32>("{10,20}"), Some(Complex { re: 10, im: 20 }));
}
fn complexP<T: FromStr>(s: &str) -> Option<Complex<T>> {
    match bracesP(s) {
        Some(t) => match pairP::<T>(t, ',') {
            Some((re, im)) => Some(Complex { re, im }),
            _ => None,
        },
        _ => None,
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn main() {
    println!("Hello, world!");
}
