use std::ops::{Add, Sub}; // <= bring these traits into local scope
use std::time::Duration; // <= bring Duration type into local scope

/**
	- lowercase name for variables eg. a, b
	- Single Uppercase letters used for generics eg. T, U, V
	- Terms with beginning Uppercase for traits eg. Add, Sub
	- "'a" for lifetime parameters eg. 'a, 'b
*/

// Generic with T is not enough, the operation performed must also be defined.
fn add<T: Add<Output = T>>(i: T, j: T) -> T{
	i + j
}

fn subtract<T: Sub<Output = T>>(i: T, j: T) -> T {
	i - j
}

fn main(){
	println!("{} + {} = {}", 10, 20, add(10, 20));
	println!("{} - {} = {}", 10, 20, subtract(10, 20));
	println!("{} + {} = {}", 10.5, 20.8, add(10.5, 20.8));
	println!("{} - {} = {}", 10.5, 20.8, subtract(10.5, 20.8));
	println!("{:?} + {:?} = {:?}", Duration::new(5, 0), Duration::new(10, 0), add(Duration::new(5, 0), Duration::new(10, 0)));
	println!("{:?} - {:?} = {:?}", Duration::new(15, 0), Duration::new(10, 0), subtract(Duration::new(15, 0), Duration::new(10, 0)));
}