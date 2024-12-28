use std::thread;

fn main() {
	let mut data = 100;
	thread::spawn(|| {data = 100;});
	thread::spawn(|| {data = 400; });
    println!("{}", data);
}
