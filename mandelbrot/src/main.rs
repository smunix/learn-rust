use image::png::PNGEncoder;
use image::ColorType;
use num::{Complex, Float};
use std::env;
use std::fs::File;
use std::str::FromStr;

fn open_close_p(s: &str, l: char, r: char) -> Option<&str> {
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

fn braces_p(s: &str) -> Option<&str> {
    open_close_p(s, '{', '}')
}

fn parens_p(s: &str) -> Option<&str> {
    open_close_p(s, '(', ')')
}

fn pair_p<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(lhs), Ok(rhs)) => Some((lhs, rhs)),
            _ => None,
        },
    }
}

#[test]
fn test_pair_p() {
    assert_eq!(pair_p::<i32>("", ','), None);
    assert_eq!(pair_p::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(pair_p::<i32>("10x20", 'x'), Some((10, 20)));
    assert_eq!(pair_p::<f32>("10x20", 'x'), Some((10.0, 20.0)));
}

#[test]
fn test_complex_p() {
    assert_eq!(
        complex_p::<f64>("{10.0,20.0}", ','),
        Some(Complex { re: 10.0, im: 20.0 })
    );

    assert_eq!(
        complex_p::<f64>("   {10.0,20.0}", ','),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complex_p::<f64>("{10.0,20.0}   ", ','),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complex_p::<i32>("{10,20}", ','),
        Some(Complex { re: 10, im: 20 })
    );
}

fn complex_p<T: FromStr>(s: &str, sep: char) -> Option<Complex<T>> {
    match braces_p(s) {
        Some(t) => match pair_p::<T>(t, sep) {
            Some((re, im)) => Some(Complex { re, im }),
            _ => None,
        },
        _ => None,
    }
}

#[test]
fn test_pixel_point() {
    assert_eq!(
        pixel_point::<f64>(
            (100, 200),
            (0, 0),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: -1.0, im: 1.0 }
    );
    assert_eq!(
        pixel_point::<f64>(
            (100, 200),
            (100, 200),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: 1.0, im: -1.0 }
    );
    assert_eq!(
        pixel_point::<f64>(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );

    assert_eq!(
        pixel_point::<f32>(
            (100, 200),
            (0, 0),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: -1.0, im: 1.0 }
    );
    assert_eq!(
        pixel_point::<f32>(
            (100, 200),
            (100, 200),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: 1.0, im: -1.0 }
    );
    assert_eq!(
        pixel_point::<f32>(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
fn pixel_point<T>(
    bounds: (usize, usize),
    pixel: (usize, usize),
    ul: Complex<T>,
    lr: Complex<T>,
) -> Complex<T>
where
    T: Float,
{
    let (w, h) = (lr.re - ul.re, ul.im - lr.im);
    let w_ratio = w / T::from(bounds.0).unwrap();
    let h_ratio = h / T::from(bounds.1).unwrap();
    Complex {
        re: ul.re + T::from(pixel.0).unwrap() * w_ratio,
        im: ul.im - T::from(pixel.1).unwrap() * h_ratio,
    }
}

fn loop_n<T>(c: Complex<T>, limit: usize) -> Option<usize>
where
    T: Float,
{
    let mut z = Complex {
        re: T::from(0.2)?,
        im: T::from(0.2)?,
    };
    for i in 0..limit {
        if z.norm_sqr() > T::from(4.0)? {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn render<T>(pixels: &mut [u8], bounds: (usize, usize), ul: Complex<T>, lr: Complex<T>)
where
    T: Float,
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let point = pixel_point::<T>(bounds, (x, y), ul, lr);
            pixels[y * bounds.0 + x] = match loop_n(point, 255) {
                Some(n) => 255 - n as u8,
                _ => 0,
            }
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let enc = PNGEncoder::new(File::create(filename)?);
    enc.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UL LR", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 (-1.20,0.35) (-1.0,0.20)",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = pair_p(&args[2], 'x').expect("error parsing image dimensions");
    let ul = complex_p::<f64>(&args[3], ',').expect("error parsing upper left corner point");
    let lr = complex_p::<f64>(&args[4], ',').expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, ul, lr);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
