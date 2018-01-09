extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

//  use /// to mark the comment lines above the function definition; the comments above the members
//  of the Complex structure start with /// as well. These are documentation comments; the rustdoc
//  utility knows how to parse them, together with the code they describe, and produce online
//  documentation. 
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
// 1. The function’s return value is an Option<u32>, for any type T, a value 
//    of type Option<T> is either Some(v), where v is a value of type T;
//    or None, indicating no T value is available.
// 2. Option is a generic type: you can use Option<T> to represent an optional
//    value of any type T you like.
//
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
	let mut z = Complex { re: 0.0, im: 0.0 };
	for i in 0..limit {
		z = z*z + c;
        //3. The z.norm_sqr() method call returns the square of z’s distance from the origin.
        //   instead of computing a square root, we just compare the squared distance with 4.0,
        //   which is faster.
		if z.norm_sqr() > 4.0 {
			return Some(i);
		}
	}

	None
}

use std::str::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
// 1.  The definition of parse_pair is a generic function
// 1.1 When you use a generic function, Rust will often be able to infer type parameters for you
// 1.2  <T: FromStr> means "For any type T that implements the FromStr trait..." 
// 2.  return type is Option<(T, T)>: either None, or a value Some((v1, v2)), where (v1, v2) is a
//     tuple of two values, both of type T.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    // 3. no explicit return statement, so its return value is the value of the last
    //    expression in the match body
	match s.find(separator) {
        //4. the entire match expression evaluates to None, indicating that the parse failed.
		None => None,
        //5. Otherwise, we take index to be the separator’s position in the string.
		Some(index) => {
            // The power of the match expression in the Rust.
            // 6.  The argument to the match is this tuple expression
            //     (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) 
            // 7.  This pattern matches only if both elements of the tuple are Ok variants of
            //     the Result type, indicating that both parses succeeded.
            // 8.  The wildcard pattern _ matches anything, and ignores its value. 
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
				(Ok(l), Ok(r)) => Some((l, r)),
				_ => None
			}
		}
	}
}

#[test]
fn test_parse_pair() {
	assert_eq!(parse_pair::<i32>("",        ','), None);
	assert_eq!(parse_pair::<i32>("10,",     ','), None);
	assert_eq!(parse_pair::<i32>(",10",     ','), None);
	assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
	assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
	assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
	assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
// 9. Complex { re, im } is a shorthand notation to build the Complex value. 
//    aka. to initialize a struct’s fields with variables of the same name
fn parse_complex(s: &str) -> Option<Complex<f64>> {
	match parse_pair(s, ',') {
		Some((re, im)) => Some(Complex { re, im }),
		None => None
	}
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(bounds: (usize, usize),
				  pixel: (usize, usize),
				  upper_left: Complex<f64>,
				  lower_right: Complex<f64>)
	-> Complex<f64>
{
	let (width, height) = (lower_right.re - upper_left.re,
						   upper_left.im - lower_right.im);
    // 10.  pixel.0 refers to the first element of the tuple pixel.
    // 11.  `as f64` is Rust’s syntax for a type conversion: this converts
    //      pixel.0 to an f64 value.
	Complex {
		re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
		im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
			// Why subtraction here? pixel.1 increases as we go down,
			// but the imaginary component increases as we go up.
	}
}

#[test]
fn test_pixel_to_point() {
	assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im:  1.0 }, 
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.5 });
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(pixels: &mut [u8],
		  bounds: (usize, usize),
		  upper_left: Complex<f64>,
		  lower_right: Complex<f64>)
{
	assert!(pixels.len() == bounds.0 * bounds.1);

	for row in 0 .. bounds.1 {
		for column in 0 .. bounds.0 {
			let point = pixel_to_point(bounds, (column, row),
			upper_left, lower_right);
			pixels[row * bounds.0 + column] =
				match escape_time(point, 255) {
					None => 0,
					Some(count) => 255 - count as u8
				};
		}
	}
}

extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
// 12.  write_image function has no useful value to return, So its success type is
//      the unit type (), so called because it has only one value. 
// 12.1 The unit type is akin to void in C and C++.
// 13.  we can use Result<()> shorthand for Result<T, std::io::Error>, if we bring it
//      into scope with a use std::io::Result declaration
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
	-> Result<(), std::io::Error>
{
    // 12. The ? operator exists to make these checks convenient. 
    //     Instead of spelling everything out like:
    //      let output = match File::create(filename) {
    //          Ok(f) => { f }
    //          Err(e) => { return Err(e); }
    //      };
	let output = File::create(filename)?;

	let encoder = PNGEncoder::new(output);
    // the value ColorType::Gray(8) indicates that each byte is an eight-bit grayscale value.
	encoder.encode(&pixels,
				   bounds.0 as u32, bounds.1 as u32,
				   ColorType::Gray(8))?;

	Ok(())
}

use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 6 {
        writeln!(std::io::stderr(),
        "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT")
            .unwrap();
        writeln!(std::io::stderr(),
        "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20 fast",
        args[0])
            .unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    // 15.  A macro call vec![v; n] creates a vector n elements long 
    //      whose elements are initialized to v
    let mut pixels = vec![0; bounds.0 * bounds.1];
    
    let mut concurrent = true;
    match &args[5][..] {
        "fast" => concurrent = true,
        "slow" => concurrent = false,
             _ => concurrent = true
    }
    // 16. The &mut pixels borrows a mutable reference to our pixel buffer, allowing
    //     render to fill it with computed grayscale values.
    //do_render(false, &mut pixels, bounds, upper_left, lower_right);
    if !concurrent {
        render(&mut pixels, bounds, upper_left, lower_right);
    }
    else{
        render_c(&mut pixels, bounds, upper_left, lower_right);
    }

    // 17. In this case, we pass a shared (nonmutable) reference &pixels , since 
    //     write_image should have no need to modify the buffer’s contents.
    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");
}

extern crate crossbeam;
fn render_c(pixels: &mut [u8],
            bounds: (usize, usize),
            upper_left: Complex<f64>,
            lower_right: Complex<f64>){
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left =
                    pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height),
                    upper_left, lower_right);

                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }
} 
