use image::png::PNGEncoder;
use image::ColorType;
use num::{Complex, Float};
use std::env;
use std::fs::File;
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
        complexP::<f64>("{10.0,20.0}", ','),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complexP::<f64>("   {10.0,20.0}", ','),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complexP::<f64>("{10.0,20.0}   ", ','),
        Some(Complex { re: 10.0, im: 20.0 })
    );
    assert_eq!(
        complexP::<i32>("{10,20}", ','),
        Some(Complex { re: 10, im: 20 })
    );
}

fn complexP<T: FromStr>(s: &str, sep: char) -> Option<Complex<T>> {
    match bracesP(s) {
        Some(t) => match pairP::<T>(t, sep) {
            Some((re, im)) => Some(Complex { re, im }),
            _ => None,
        },
        _ => None,
    }
}

#[test]
fn test_pixelPoint() {
    assert_eq!(
        pixelPoint::<f64>(
            (100, 200),
            (0, 0),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: -1.0, im: 1.0 }
    );
    assert_eq!(
        pixelPoint::<f64>(
            (100, 200),
            (100, 200),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: 1.0, im: -1.0 }
    );
    assert_eq!(
        pixelPoint::<f64>(
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
        pixelPoint::<f32>(
            (100, 200),
            (0, 0),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: -1.0, im: 1.0 }
    );
    assert_eq!(
        pixelPoint::<f32>(
            (100, 200),
            (100, 200),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: 1.0, im: -1.0 }
    );
    assert_eq!(
        pixelPoint::<f32>(
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
fn pixelPoint<T>(
    bounds: (usize, usize),
    pixel: (usize, usize),
    ul: Complex<T>,
    lr: Complex<T>,
) -> Complex<T>
where
    T: Float,
{
    let (w, h) = (lr.re - ul.re, ul.im - lr.im);
    let wRatio = w / T::from(bounds.0).unwrap();
    let hRatio = h / T::from(bounds.1).unwrap();
    Complex {
        re: ul.re + T::from(pixel.0).unwrap() * wRatio,
        im: ul.im - T::from(pixel.1).unwrap() * hRatio,
    }
}

fn loopN<T>(c: Complex<T>, limit: usize) -> Option<usize>
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
            let point = pixelPoint::<T>(bounds, (x, y), ul, lr);
            pixels[y * bounds.0 + x] = match loopN(point, 255) {
                Some(n) => 255 - n as u8,
                _ => 0,
            }
        }
    }
}

fn writeImage(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
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

    let bounds = pairP(&args[2], 'x').expect("error parsing image dimensions");
    let ul = complexP::<f64>(&args[3], ',').expect("error parsing upper left corner point");
    let lr = complexP::<f64>(&args[4], ',').expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, ul, lr);
    writeImage(&args[1], &pixels, bounds).expect("error writing PNG file");
}
