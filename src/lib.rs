//! A lossy representation of a path using a `String` for quick and dirty fuzzy comparison.
//!
//! This type deliberately does not implement `PartialEq` for any other types. You should only compare `FuzzyPath` with another `FuzzyPath`, as the normalization must take place.
//!
//! # Comparison rules
//!
//! * ✅ Case insensitive
//! * ✅ Backslashes are normalized to forward slashes
//! * ✅ Trailing slashes are removed, except for root slash (for absolute POSIX paths)
//! * ✅ Repeating slashes are normalized to a single slash
//! * ❌ Comparing a Windows path with a POSIX path will not work if either is absolute (Windows paths with a drive letter, POSIX paths with a preceeding slash)
//! * ❌ Comparing a Windows UNC path will not work with any POSIX path

use std::{borrow::Borrow, path::PathBuf, str::FromStr};

/// A lossy representation of a path using a `String` for quick and dirty fuzzy comparison.
///
/// This type deliberately does not implement `PartialEq` for any other types. You should only compare `FuzzyPath` with another `FuzzyPath`, as the normalization must take place.
///
/// # Comparison rules
///
/// * ✅ Case insensitive
/// * ✅ Backslashes are normalized to forward slashes
/// * ✅ Trailing slashes are removed, except for root slash (for absolute POSIX paths)
/// * ✅ Repeating slashes are normalized to a single slash
/// * ❌ Comparing a Windows path with a POSIX path will not work if either is absolute (Windows paths with a drive letter, POSIX paths with a preceeding slash)
/// * ❌ Comparing a Windows UNC path will not work with any POSIX path
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct FuzzyPath(String);
impl FuzzyPath {
	/// It is a logic error to construct a `FuzzyPath` from a string that is not correctly normalized.
	///
	/// To see the normalization implementation, see the `From<&str>` implementation for `FuzzyPath`
	pub unsafe fn from_str_unchecked<S: Into<String>>(str: S) -> Self {
		FuzzyPath(str.into())
	}
}
impl From<&str> for FuzzyPath {
	fn from(str: &str) -> Self {
		let str = str.replace("\\", "/"); // Normalize backslashes to forward slashes
		let str = str.trim_end_matches("/"); // Trim trailing slashes

		// Find and obliterate repeating slashes
		let mut normalized = String::with_capacity(str.len());
		let mut slash = false;
		for char in str.chars() {
			if char == '/' {
				if !slash {
					slash = true;
					normalized.push('/');
				}
			} else {
				slash = false;
				let char = char.to_lowercase(); // Normalize to lowercase
				normalized.extend(char);
			}
		}

		FuzzyPath(normalized)
	}
}
impl From<PathBuf> for FuzzyPath {
	fn from(pathbuf: PathBuf) -> Self {
		pathbuf.to_string_lossy().as_ref().into()
	}
}
impl From<String> for FuzzyPath {
	fn from(string: String) -> Self {
		string.as_str().into()
	}
}
impl Into<PathBuf> for FuzzyPath {
	fn into(self) -> PathBuf {
		PathBuf::from(self.0)
	}
}
impl Into<String> for FuzzyPath {
	fn into(self) -> String {
		self.0
	}
}
impl AsRef<String> for FuzzyPath {
	fn as_ref(&self) -> &String {
		&self.0
	}
}
impl AsRef<str> for FuzzyPath {
	fn as_ref(&self) -> &str {
		self.0.as_str()
	}
}
impl Borrow<str> for FuzzyPath {
	fn borrow(&self) -> &str {
		self.0.as_str()
	}
}
impl Borrow<String> for FuzzyPath {
	fn borrow(&self) -> &String {
		&self.0
	}
}
impl FromStr for FuzzyPath {
	type Err = (); // Maybe use `!` when it's stabilized?

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(s.into())
	}
}

#[cfg(feature = "serde")]
mod serde;

#[test]
fn test_normalize() {
	assert_eq!(FuzzyPath::from("HELLO\\\\world/////foo/bar/////////"), FuzzyPath::from("hello/world/foo/bar"));
	assert_eq!(FuzzyPath::from("\\\\HELLO\\\\world/////foo/bar/////////"), FuzzyPath::from("/hello/world/foo/bar"));
}