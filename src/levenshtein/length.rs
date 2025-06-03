#[derive(Debug, Clone, Copy)]
pub enum LengthError {
	Overflow,
}

#[inline(always)]
pub fn unicode_difference(s1: &str, s2: &str) -> (usize, usize, isize) {
	try_length_difference(s1, s2, true).expect("byte stream exceeds isize::MAX bytes")
}

#[inline(always)]
pub fn byte_difference(s1: &str, s2: &str) -> (usize, usize, isize) {
	try_length_difference(s1, s2, false).expect("byte stream exceeds isize::MAX bytes.")
}

pub fn try_length_difference(s1: &str, s2: &str, unicode: bool) -> Result<(usize, usize, isize), LengthError> {
	// Returns lenght and difference of two strings as a Result() -> handle with Ok() and Err() pattern.
	// diff can be negative if s1 is greater than s2.
	// lenght of s1 and s2 can not exceed isize::MAX to prevent overflow
	let (s1_len, s2_len): (usize, usize) = if unicode {
		(s1.chars().count(), s2.chars().count())
	} else {
		(s1.len(), s2.len())
	};

    // Note: usize::MAX is always ≤ isize::MAX * 2 + 1, so this is safe
    if s1_len > isize::MAX as usize || s2_len > isize::MAX as usize {
        return Err(LengthError::Overflow);
    }

    let diff = s2_len as isize - s1_len as isize;
    Ok((s1_len, s2_len, diff))
}