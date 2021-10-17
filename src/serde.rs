use serde::{*, de::Visitor};

use crate::FuzzyPath;

struct FuzzyPathVisitor;
impl<'v> Visitor<'v> for FuzzyPathVisitor {
	type Value = FuzzyPath;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a string")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where E: serde::de::Error
	{
		Ok(FuzzyPath::from(v))
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: de::Error
	{
		Ok(FuzzyPath::from(v))
	}
}

impl<'de> Deserialize<'de> for FuzzyPath {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		deserializer.deserialize_str(FuzzyPathVisitor)
	}
}