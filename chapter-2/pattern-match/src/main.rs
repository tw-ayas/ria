fn main() {
	let needle  = 42;
	let haystack = [1,1,2,3,4,5,1,42,123, 123];
	
	for item in &haystack {
		if *item == needle {
			println!("item found")
		}
		// let result = match item {
		// 	42 | 132 => "hit!",
		// 	_ => "miss",
		// };
		
		// if result == "hit!" {
		// 	println!("{}: {}", item, result);
		// }
	}
}
