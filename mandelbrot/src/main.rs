use num::Complex;
use std::str::FromStr;

fn parse_open_close(s: &str, l: char, r: char) -> Option<&str> {
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

fn parse_braces(s: &str) -> Option<&str> {
    parse_open_close(s, '{', '}')
}

fn parse_parens(s: &str) -> Option<&str> {
    parse_open_close(s, '(', ')')
}

fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(lhs), Ok(rhs)) => Some((lhs, rhs)),
            _ => None,
        },
    }
}

#[test]
fn test_pair_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
    assert_eq!(parse_pair::<f32>("10x20", 'x'), Some((10.0, 20.0)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex::<f64>("{10.0,20.0}"),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        parse_complex::<f64>("   {10.0,20.0}"),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        parse_complex::<f64>("{10.0,20.0}   "),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        parse_complex::<i32>("{10,20}"),
        Some(Complex { re: 10, im: 20 })
    );
}
fn parse_complex<T: FromStr>(s: &str) -> Option<Complex<T>> {
    match parse_braces(s) {
        Some(t) => match parse_pair::<T>(t, ',') {
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
