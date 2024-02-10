fn main() {
	let symbol:char = 'x';
	println!("Character is: {}", symbol);
	println!("Size of char value {} is {} bytes", symbol, std::mem::size_of_val(&symbol));
}