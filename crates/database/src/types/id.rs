use std::{
	borrow::Cow,
	fmt,
	str::FromStr,
	time::{SystemTime, UNIX_EPOCH},
};

use base64::{
	engine::{general_purpose::URL_SAFE_NO_PAD, Engine},
	DecodeError,
};
use rand::{rngs::OsRng, RngCore};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// A Database Kind
///
/// 2 bytes / 16 bits
/// +---------+----+
/// |component|kind|
/// +---------+----+
/// |    1    | 15 |
/// +---------+----+
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kind([u8; 2]);

impl Kind {
	pub fn new(component: bool, kind: u16) -> Self {
		let mut bytes = [0u8; 2];
		let kind_bytes = kind.to_be_bytes();

		// check that the first bit is not set of kind
		assert_eq!(kind_bytes[0] & 0b1000_0000, 0, "Kind has first bit set!");
		if component {
			bytes[0] |= 0b1000_0000;
		}

		bytes[0] |= kind_bytes[0];
		bytes[1] = kind_bytes[1];

		Self(bytes)
	}

	pub fn is_component(&self) -> bool {
		self.0[0] & 0b1000_0000 != 0
	}

	pub fn kind(&self) -> u16 {
		u16::from_be_bytes([self.0[0] & 0b0111_1111, self.0[1]])
	}
}

/// A Database Id
///
///  12 bytes / 104 bits
/// +----+------+----+  
/// |secs|random|kind|  
/// +----+------+----+  
/// | 40 |  40  | 16 |  
/// +----+------+----+  
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id([u8; 12]);

impl Id {
	/// Create a new Id based on the kind
	pub fn new(kind: Kind) -> Self {
		let secs_bytes = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("SystemTime before UNIX EPOCH!")
			.as_secs()
			.to_be_bytes();

		let mut bytes = [0u8; 12];
		bytes[..5].copy_from_slice(&secs_bytes[3..8]);

		OsRng.fill_bytes(&mut bytes[5..11]);

		bytes[10] = kind.0[0];
		bytes[11] = kind.0[1];

		Self(bytes)
	}

	/// Returns the kind of the id
	pub fn kind(&self) -> Kind {
		Kind([self.0[10], self.0[11]])
	}

	/// Returns the id as a base64 string
	pub fn to_b64(&self) -> String {
		URL_SAFE_NO_PAD.encode(self.0)
	}

	/// If the string is not 16 bytes long returns InvalidLengthu
	pub fn parse_b64<T>(b64: T) -> Result<Self, DecodeError>
	where
		T: AsRef<[u8]>,
	{
		if b64.as_ref().len() != 16 {
			return Err(DecodeError::InvalidLength(b64.as_ref().len()));
		}

		let mut bytes = [0u8; 12];
		URL_SAFE_NO_PAD
			.decode_slice_unchecked(b64, &mut bytes)
			.map(|n| assert_eq!(n, bytes.len()))
			.map(|_| Self(bytes))
	}

	pub fn from_bytes(bytes: [u8; 12]) -> Self {
		Self(bytes)
	}

	pub fn into_bytes(self) -> [u8; 12] {
		self.0
	}

	pub fn as_slice(&self) -> &[u8] {
		&self.0
	}
}

impl fmt::Debug for Id {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("Id").field(&self.to_b64()).finish()
	}
}

impl fmt::Display for Id {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.to_b64().fmt(f)
	}
}

impl FromStr for Id {
	type Err = DecodeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::parse_b64(s)
	}
}

impl From<Id> for Value {
	fn from(id: Id) -> Self {
		Value::String(id.to_string())
	}
}

impl Serialize for Id {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_b64())
	}
}

impl<'de> Deserialize<'de> for Id {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: Cow<'_, str> = Deserialize::deserialize(deserializer)?;
		Id::parse_b64(s.as_ref()).map_err(D::Error::custom)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn kind() {
		let kind = Kind::new(true, 0x7FFF);
		assert_eq!(kind.is_component(), true);
		assert_eq!(kind.kind(), 0x7FFF);

		let kind = Kind::new(false, 0x7FFF);
		assert_eq!(kind.is_component(), false);
		assert_eq!(kind.kind(), 0x7FFF);
	}

	#[test]
	fn id() {
		let kind = Kind::new(true, 0x7FFF);
		let id = Id::new(kind);
		assert_eq!(id.kind(), kind);
	}

	#[test]
	fn b64() {
		let kind = Kind::new(true, 0x7FFF);
		let id = Id::new(kind);
		let b64 = id.to_b64();
		let id2 = Id::parse_b64(b64).unwrap();
		assert_eq!(id, id2);
	}

	// check max length of b64 parse
	#[test]
	fn b64_max() {
		let input = "aHR0cHM6Ly9naXRZ";
		let id = Id::parse_b64(input).unwrap();
		assert_eq!(id.to_b64(), input);
	}
}
