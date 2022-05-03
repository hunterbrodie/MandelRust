extern crate image;
use image::ImageBuffer;

use std::env;

use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let args: Vec<String> = env::args().collect();
	let width_steps = args[1].parse::<u32>()?;

	let data = calculate_set(width_steps);

	write_to_image(data);

	Ok(())
}

fn write_to_image(data: Vec<Vec<u8>>)
{
	let img = ImageBuffer::from_fn(data.len() as u32, data[0].len() as u32, |x, y|
	{
		let color = data[x as usize][y as usize];
		if color == 32
		{
			image::Rgb([0u8, 0u8, 0u8])
		}
		else
		{
			image::Rgb([(data[x as usize][y as usize] as f32 / 32f32 * 255f32) as u8, 127u8, 127u8])
		}
	});

	img.save("mandelbrot.png").unwrap();
}

fn calculate_set(width_steps: u32) -> Vec<Vec<u8>>
{
	let increment: f32 = 2.7 / (width_steps as f32);
	let height_steps: u32 = (2.4 / increment).ceil() as u32;
	let mut data: Vec<Vec<u8>> = Vec::new();

	let mut handles: Vec<std::thread::JoinHandle<Vec<u8>>> = Vec::new();

	let mut threads = 0;

	for z in 0..width_steps
	{
		let j = z as f32 * increment - 2.1;

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

			for i in 0..height_steps
			{
				let k = i as f32 * increment - 1.2;
	
				row.push(mandelbrot(0.0, 0.0, j, k, 0));
			}

			row
		}));
	}

	for handle in handles
	{
		data.push(handle.join().unwrap());
	}

	data
}

fn mandelbrot(z: f32, i: f32, z0: f32, i0: f32, n: u8) -> u8
{
	let i_n: f32 = 2.0 * z * i + &i0;
	let z_n: f32 = z.powi(2) - i.powi(2) + &z0;
	if bounded(z_n, i_n)
	{
		if n == 32
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

fn bounded(z: f32, i: f32) -> bool
{
	let mut dist: f32 = z.powi(2) + i.powi(2);
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