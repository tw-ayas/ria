// 'a and 'b are lifetime variable declaration
// lifetime variable 'a is bounded to lifetime of i. "parameter i is a reference to an i32 with lifetime a"
fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32{
	*i + *j
}



fn main() {
	let a = 10;
	let b = 20;
	let result = add_with_lifetime(&a, &b);
	println!("{}", result);
}
