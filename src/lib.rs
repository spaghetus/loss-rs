use std::{fmt::Debug, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum LossBits {
	I = 0,
	Ii = 1,
	II = 2,
	I_ = 3,
}
impl Default for LossBits {
	fn default() -> Self {
		LossBits::I
	}
}

impl LossBits {
	pub fn from_bits(bits: [bool; 2]) -> LossBits {
		match bits {
			[false, false] => LossBits::I,
			[false, true] => LossBits::Ii,
			[true, false] => LossBits::II,
			[true, true] => LossBits::I_,
		}
	}
	pub fn from_byte(byte: u8) -> [LossBits; 4] {
		let mut list: [LossBits; 4] = Default::default();
		for (i, n) in list.iter_mut().enumerate() {
			let shr = byte >> (i * 2);
			let a = (shr & 2) > 0;
			let b = (shr & 1) > 0;
			*n = LossBits::from_bits([a, b])
		}
		list
	}
	pub fn to_bits(&self) -> [bool; 2] {
		match self {
			LossBits::I => [false, false],
			LossBits::Ii => [false, true],
			LossBits::II => [true, false],
			LossBits::I_ => [true, true],
		}
	}
	pub fn to_byte(input: [LossBits; 4]) -> u8 {
		let mut out = 0;
		for (i, n) in input.iter().enumerate() {
			let [a, b] = n.to_bits();
			let a = if a { 2 } else { 0 };
			let b = if b { 1 } else { 0 };
			let c = a | b;
			out |= c << (i * 2);
		}
		out
	}
}

impl FromStr for LossBits {
	fn from_str(string: &str) -> Result<LossBits, ()> {
		Ok(match string {
			"I" => Self::I,
			"Ii" => Self::Ii,
			"II" => Self::II,
			"I_" => Self::I_,
			_ => unreachable!(),
		})
	}

	type Err = ();
}

impl ToString for LossBits {
	fn to_string(&self) -> String {
		match self {
			Self::I => "I".to_string(),
			Self::Ii => "Ii".to_string(),
			Self::II => "II".to_string(),
			Self::I_ => "I_".to_string(),
		}
	}
}

#[test]
fn test_lossbits() {
	for n in 0..u8::MAX {
		let loss = LossBits::from_byte(n);
		println!("{:?}", loss);
		let out = LossBits::to_byte(loss);
		assert_eq!(out, n)
	}
}
