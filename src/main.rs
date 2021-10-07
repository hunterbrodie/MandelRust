extern crate image;
use image::{ImageBuffer};

use std::env;

use std::thread;

fn main()
{
	let args: Vec<String> = env::args().collect();
	let sections = args[1].parse::<i32>().unwrap();

	let step: f64 = 2.1 / (sections as f64);
	let mut data: Vec<Vec<u8>> = Vec::new();

	let mut handles: Vec<std::thread::JoinHandle<Vec<u8>>> = Vec::new();

	let mut threads = 0;

	for z in -sections..sections
	{
		let j = f64::from(z) * step;

		threads += 1;

		if threads % 8 == 0
		{
			for handle in handles
			{
				data.push(handle.join().unwrap());
			}

			handles = Vec::new();
		}

		handles.push(thread::spawn(move ||
		{
			let mut row: Vec<u8> = Vec::new();

			for i in -sections..sections
			{
				let k = f64::from(i) * step;
	
				row.push(mandelbrot(0.0, 0.0, j, k, 0));
			}

			row
		}));
	}

	for handle in handles
	{
		data.push(handle.join().unwrap());
	}

	println!("Done Calculating Mandelbrot Set!");

	write_to_image(data);

	println!("Done Writing Mandelbrot Set!");
}

fn write_to_image(data: Vec<Vec<u8>>)
{
	let img = ImageBuffer::from_fn(data.len() as u32, data[0].len() as u32, |x, y|
	{
		image::Luma([data[x as usize][y as usize]])
	});

	img.save("mandelbrot.png").unwrap();
}

fn mandelbrot(z: f64, i: f64, z0: f64, i0: f64, n: u8) -> u8
{
	let i_n: f64 = 2.0 * z * i + &i0;
	let z_n: f64 = z.powi(2) - i.powi(2) + &z0;
	if bounded(z_n, i_n)
	{
		if n == 255
		{
			return n
		}
		else
		{
			mandelbrot(z_n, i_n, z0, i0, n + 1)
		}
	}
	else
	{
		return n
	}
}

fn bounded(z: f64, i: f64) -> bool
{
	let mut dist: f64 = z.powi(2) + i.powi(2);
	dist = dist.sqrt();
	if dist < 16.0
	{
		true
	}
	else
	{
		false
	}
}