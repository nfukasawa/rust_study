extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT",
            args[0]
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let h = band.len() / bounds.0;
                let band_bounds = (bounds.0, h);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + h), upper_left, lower_right);
                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn write_image(
    filename: &str,
    pixcels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let out = File::create(filename)?;
    let enc = PNGEncoder::new(out);
    enc.encode(
        &pixcels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let p = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(p, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (w, h) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * w / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re: re, im: im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
