use std::io;
use rand::Rng;
use rand::thread_rng;

fn main() {
	let mut rng = thread_rng();
	let random_number_in_range: u32 = rng.gen_range(-1..=37);
	
	println!("Guess the number!\n(Hint: it's between -1 and 37)");

	println!("Please input your guess.");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

    println!("You guessed: {guess}");
}
