
pub fn stem_byte_sequence(s: &str) -> Vec<(char, usize)> {
	let mut freq: Vec<(char, usize)> = Vec::new();
	for c in s.bytes() {
		let c = c as char;
		match freq.iter_mut().find(|(ch, _)| *ch == c) {
			Some((_, count)) => *count += 1,
			None => freq.push((c,1)),
		}
	}
	freq
}

pub fn stem_unicode_sequence(s: &str) -> Vec<(char, usize)> {
	// Treats s as unicode and returns a frequency table
	let mut freq: Vec<(char, usize)> = Vec::new();
	for c in s.chars() {
		match freq.iter_mut().find(|(ch, _)| *ch == c) {
			Some((_, count)) => *count += 1,
			None => freq.push((c, 1)),
		}
	}
	freq
}