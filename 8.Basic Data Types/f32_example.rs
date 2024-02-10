fn main() {
	let num1:f32 = 1.1;
	let num2:f32 = 1.2;
	let added_numbers:f32 =num1+num2;
	println!("Value is {}", added_numbers);
	println!("Size of f32 value {} is {} bytes", added_numbers, std::mem::size_of_val(&added_numbers));
}