use std::io;
use std::io::prelude::*; 

fn main()
{
	for z in -235..85
	{
		let j = f64::from(z) * 0.01;
		for i in -16..16
		{
			let k = f64::from(i) * 0.1;
			if mandelbrot(0.0, 0.0, j, k, 0)
			{
				print!("*");
			}
			else
			{
				print!(" ");
			}
			io::stdout().flush().unwrap();
		}
		println!();
	}
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