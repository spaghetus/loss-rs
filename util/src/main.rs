use lossrs_util::LossBits;
use rayon::prelude::*;

fn main() {
	let src = std::env::args().nth(1).expect("required source file");
	let out = std::env::args().nth(2).expect("required destination file");

	let in_file: Vec<u8> = std::fs::read(src).expect("couldn't read in");
	let in_losses: Vec<_> = in_file.iter().map(|v| LossBits::from_byte(*v)).collect();
	let string = in_losses
		.par_iter()
		.flatten()
		.map(|v| v.to_string())
		.collect::<Vec<_>>()
		.join(" ");
	let string = "lossrs_derive::losscode! {".to_string() + &string + "}";
	std::fs::write(out, string.as_bytes()).expect("couldn't write out");
}
