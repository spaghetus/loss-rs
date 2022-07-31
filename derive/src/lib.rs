use lossrs_util::LossBits;
use proc_macro::TokenStream;
use rayon::prelude::*;
use std::str::FromStr;

#[proc_macro]
pub fn losscode(ts: TokenStream) -> TokenStream {
	let lossbits = ts
		.into_iter()
		.map(|v| match v {
			proc_macro::TokenTree::Ident(s) => {
				let s = s.to_string();
				lossrs_util::LossBits::from_str(&s).unwrap()
			}
			_ => unimplemented!(),
		})
		.collect::<Vec<_>>();
	let bytes = lossbits
		.par_chunks_exact(4)
		.map(|v| {
			let mut lossbits: [LossBits; 4] = Default::default();
			lossbits.copy_from_slice(v);
			LossBits::to_byte(lossbits)
		})
		.collect();
	let string = String::from_utf8(bytes).unwrap();
	TokenStream::from_str(&string).unwrap()
}
