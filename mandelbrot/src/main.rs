extern crate num;
extern crate image;
extern crate crossbeam;

use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;

fn main() 
{
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 5 
	{
		writeln!(std::io::stderr(), "Usage: mandelbrot FILE PIXELS_BOUNDING_BOX_SIZE UPPER_LEFT LOWER_RIGHT").unwrap();
		writeln!(std::io::stderr(), "Example: {} mandel.png 1000 -1.20,0.35 -1,0.20", args[0]).unwrap();

		std::process::exit(1);
	}

	let output = &args[1];
	let bounding_box_size = usize::from_str(&args[2]).expect("error parsing image bounding box size");
	let upper_left = parse_complex(&args[3],',').expect("error upper left");
	let lower_right = parse_complex(&args[4],',').expect("error lower right");
	let bounds = calculate_image_bounds(bounding_box_size, upper_left, lower_right);
	let mut pixels = vec![0; bounds.0 * bounds.1];

	let threads = 8;
	let rows_per_band = bounds.1 / threads + 1;

	{
		let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
		crossbeam::scope(|spawner| 
		{
			for(i, band) in bands.into_iter().enumerate()
			{
				let top = rows_per_band * i;
				let height = band.len() / bounds.0;
				let band_bounds = (bounds.0, height);
				let band_upper_left = pixel_to_point(bounds, (0,top), upper_left, lower_right);
				let band_lower_right = pixel_to_point(bounds,(bounds.0, top+height), upper_left, lower_right);

				spawner.spawn(move || {
					render(band, band_bounds, band_upper_left, band_lower_right);
				});
			}
		});
	}

	write_image(output, &pixels, bounds).expect("Error while writing PNG file");
}

fn calculate_image_bounds(bounding_box_size: usize, upper_left:Complex<f64>, lower_right:Complex<f64>) -> (usize, usize)
{
	let x_size = f64::abs(upper_left.re - lower_right.re);
	let y_size = f64::abs(upper_left.im - lower_right.im);

	if  x_size > y_size
	{
		let y_pixel = y_size / x_size;

		(bounding_box_size, (bounding_box_size as f64 * y_pixel) as usize)
	} 
	else 
	{
		let x_pixel = x_size / y_size;
		
		((bounding_box_size as f64 * x_pixel) as usize, bounding_box_size)
	}
}

/// Tries to deduce if the number is inside the Mandelbrot set, using at most 'limit' iterations to decide.
///
/// If c is not a member, returns the number of iterations needed to escape the circle of radius 2.
/// If c looks like a member, returns None.
fn escape_time (c: Complex<f64>, limit: u32) -> Option<u32>
{
	let mut z = Complex {re:0.0, im: 0.0};
	for i in 0..limit
	{
		z = z*z+c;
		if z.norm_sqr() > 4.0 { return Some(i); }
	}

	None
}


fn parse_pair<T: FromStr> (s : &str, separator:char) -> Option<(T,T)>
{
	match s.find(separator)
	{
		None=>None,
		Some(index) => 
		{
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) 
			{		
				(Ok(l), Ok(r)) => Some ((l,r)),
				_ => None
			}
		}
	}
}

fn parse_complex (s: &str, separator:char) -> Option<Complex<f64>>
{
	match parse_pair(s, separator)
	{
		Some((re, im)) => Some( Complex { re, im } ),
		None => None
	}
}

fn pixel_to_point(bounds: (usize,usize), 
				  pixel: (usize, usize), 
				  upper_left: Complex<f64>,
				  lower_right: Complex<f64>) -> Complex<f64>
{
	let (width, height) = (lower_right.re - upper_left.re, 
							upper_left.im - lower_right.im);

	Complex {
		re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
		im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
	}
}

fn render(pixels: &mut[u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>)
{
	assert!(pixels.len() == bounds.0 * bounds.1);

	for row in 0..bounds.1 
	{
		for column in 0..bounds.0
		{
			let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
			pixels[row * bounds.0 + column] = 
			match escape_time(point, 255)
			{
				None => 0,
				Some(count) => 255 - count as u8
			};
		}
	}
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize,usize)) -> Result<(), std::io::Error>
{
	let output = File::create(filename)?;
	let encoder = PNGEncoder::new(output);
	encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
	
	Ok(())
}

#[test]
fn test_parse_pair()
{
	assert_eq!(parse_pair::<i32>("", 			','), None);
	assert_eq!(parse_pair::<i32>("10,", 		','), None);
	assert_eq!(parse_pair::<i32>(",10", 		','), None);
	assert_eq!(parse_pair::<i32>("10,20", 		','), Some((10,20)));
	assert_eq!(parse_pair::<i32>("10,20xy", 	','), None);
	assert_eq!(parse_pair::<f64>("0.5x", 		'x'), None);
	assert_eq!(parse_pair::<f64>("0.5x1.5", 	'x'), Some((0.5,1.5)));
}


#[test]
fn test_parse_complex()
{
	assert_eq!(parse_complex("1.23,-0.012", ','), Some(Complex{ re:1.23, im:-0.012 }));
	assert_eq!(parse_complex(",-0.012", ','), None);
}