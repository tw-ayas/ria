use num::complex::Complex;

fn calculate_mandelbrot(
	max_iters: usize,
	x_min: f64,
	x_max: f64,
	y_min: f64,
	y_max: f64,
	width: usize,
	height: usize,
) -> Vec<Vec<usize>>
{
	let cx_h = x_max - x_min;
	let cy_h = y_max - y_min;
	let mut rows: Vec<Vec<usize>> = Vec::with_capacity(width);
	for img_y in 0..height {
		let mut row: Vec<usize> = Vec::with_capacity(height);
		for img_x in 0..width {
			let x_percent = img_x as f64 / width as f64;
			let y_percent = img_y as f64 / height as f64;
			let cx = x_min + cx_h * x_percent;
			let cy = y_min + cy_h * y_percent;
			let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
			// println!("{} {} {}", cx, cy, escaped_at);
			row.push(escaped_at);
		}
		rows.push(row);
	}
	rows
}

fn mandelbrot_at_point(
	cx: f64,
	cy: f64,
	max_iters: usize,
) -> usize {
	let mut z = Complex { re: 0.0, im: 0.0};
	let c = Complex::new(cx, cy);
	for i in 0..=max_iters {
		// println!("{} {}", z, z.norm());
		if z.norm() > 2.0 {
			return i;
		}
		z = z * z + c;
	}
	max_iters
}

fn render_mandelbrot(mandelbrot: Vec<Vec<usize>>) {
	for row in mandelbrot {
		let mut line = String::with_capacity(row.len());
		for column in row {
			let value = match column {
				0 ..=2 => '?',
				3 ..=5 => '.',
				6 ..=10 => 'o',
				11 ..=30 => '*',
				31 ..=100 => '+',
				101 ..=200 => 'x',
				201 ..=400 => '$',
				401 ..=700 => '#',
				_ => '/'
			};
			// println!("{}", column);
			line.push(value);
		}
		println!("{}", line);
	}
}

fn main() {
	let mandelbrot = calculate_mandelbrot(1000, -1.5, 1.0, -1.0, 1.0, 350, 64);
	render_mandelbrot(mandelbrot);
}
