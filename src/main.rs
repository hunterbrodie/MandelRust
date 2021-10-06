use std::fs::File;
use std::io;
use std::io::prelude::*; 

fn main()
{
	let radius = 100;
	let step: f64 = 2.0 / (80 as f64);
	let mut data: Vec<String> = Vec::new();
	for z in -radius..radius
	{
		let j = f64::from(z) * step;
		let mut row = String::new();
		for i in -radius..radius
		{
			let k = f64::from(i) * step;
			if mandelbrot(0.0, 0.0, k, j, 0)
			{
				row.push_str("*");
			}
			else
			{
				row.push_str(" ");
			}
			io::stdout().flush().unwrap();
		}
		row = String::from(row.trim_end());
		data.push(row);
	}

	let mut data_string = String::new();
	for s in data.iter()
	{
		if s.trim().is_empty() == false
		{
			data_string.push_str(&s);
			data_string.push_str("\n");
		}
	}
	
	let mut file = File::create("mandelbrot.dat").unwrap();
	file.write_all(data_string.as_bytes()).unwrap();
}

fn mandelbrot(z: f64, i: f64, z0: f64, i0: f64, n: i32) -> bool
{
	let i_n: f64 = 2.0 * z * i + &i0;
	let z_n: f64 = z.powi(2) - i.powi(2) + &z0;
	if bounded(z_n, i_n)
	{
		if n == 100
		{
			true
		}
		else
		{
			mandelbrot(z_n, i_n, z0, i0, n + 1)
		}
	}
	else
	{
		return false
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