use philib_rust::levenshtein;


fn main() {
	println!("Levenshtein Algorithm");
	let s1 = "hello 😊!";
	let s2 = String::from("hello 🧠 world");
	println!("'{}' vs. '{}'", s1, s2);
	// Length and difference
	let result_byte: (usize, usize, isize) = levenshtein::length::byte_difference(&s1, &s2);
	let result_unicode: (usize, usize, isize) = levenshtein::length::unicode_difference(&s1, &s2);
	println!("{result_byte:?}");
	println!("{result_unicode:?}");
	// Stemming string
	let stem = levenshtein::stem::stem_unicode_sequence(&s2);
	println!("{stem:?}");
	let stem = levenshtein::stem::stem_byte_sequence(&s2);
	println!("{stem:?}");
	// Levenshtein Distance
	let ldist_unicode = levenshtein::distance::unicode_edit_distance(&s1, &s2);
	println!("Unicode Levenshtein Distance: {ldist_unicode}");
	let ldist_bytes = levenshtein::distance::byte_edit_distance(&s1, &s2);
	println!("Byte Levenshtein Distance: {ldist_bytes}");
}