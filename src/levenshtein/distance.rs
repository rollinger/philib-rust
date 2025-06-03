pub fn unicode_edit_distance(s1: &str, s2: &str) -> usize {
	let mut distance: usize = 0;
	let mut s1_chars = s1.chars();
	let mut s2_chars = s2.chars();
	let max_length = s1.chars().count().max(s2.chars().count());

	for _ in 0..max_length {
		let c1 = s1_chars.next();
		let c2 = s2_chars.next();
		if c1 != c2 {
			distance += 1;
		}
	}

	distance
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance_equal_len() {
		let source = "hello";
		let alter1 = "hello";
		assert_eq!(unicode_edit_distance(&source, &alter1), 0);
		let alter2 = "Hello";
		assert_eq!(unicode_edit_distance(&source, &alter2), 1);
		let alter3 = "helloabcdef";
		assert_eq!(unicode_edit_distance(&source, &alter3), 6);
		let alter4 = "hey";
		assert_eq!(unicode_edit_distance(&source, &alter4), 3);
		let alter5 = "hello 😊!";
		assert_eq!(unicode_edit_distance(&source, &alter5), 3);

	}
}